use std::sync::{Mutex, MutexGuard};

use camino::{Utf8Path, Utf8PathBuf};

use super::command::CommandInfo;
use super::errors::Result;

/// A subdir of `target/` that cargo helpfully defines for us to scribble in during tests.
/// We are 100% responsible for its contents.
const TARGET_TEMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");

/// Top-level type that should be used to declare `statics` that define test repos
pub struct TestContextLock<Tools: 'static> {
    repo: &'static Repo,
    tools: &'static Mutex<Option<Tools>>,
    ctx: Mutex<Option<RawTestContext>>,
}
/// Inner state of a TestContext
pub struct RawTestContext {
    pub repo: &'static Repo,
    pub repo_id: String,
    pub repo_dir: Utf8PathBuf,
    pub working_dir: Utf8PathBuf,
}
/// Context passed down to test runs
pub struct TestContext<'a, Tools> {
    raw_ctx: &'a RawTestContext,
    pub tools: &'a Tools,
}
impl<'a, Tools> std::ops::Deref for TestContext<'a, Tools> {
    type Target = RawTestContext;
    fn deref(&self) -> &Self::Target {
        self.raw_ctx
    }
}
/// Info about a repo (assumed to be a github repo)
pub struct Repo {
    pub repo_owner: &'static str,
    pub repo_name: &'static str,
    /// Can be a commit SHA, tag, or branch
    pub commit_ref: &'static str,
    /// subdir where the oranda.json is
    pub subdir: Option<&'static str>,
    /// name of the application (crate)
    pub app_name: &'static str,
    /// names of binaries the application should have
    pub bins: &'static [&'static str],
}

pub trait ToolsImpl: Default {
    /// Get an implementation of `git`
    fn git(&self) -> &CommandInfo;
}

impl<Tools> TestContextLock<Tools>
where
    Tools: ToolsImpl,
{
    /// Create a new test with the given tools/repo
    ///
    /// Note that you should only have one Tools instance in your test suite, as it serves as a global
    /// lock for global mutable state like `set_current_dir`.
    pub const fn new(tools: &'static Mutex<Option<Tools>>, repo: &'static Repo) -> Self {
        Self {
            repo,
            tools,
            ctx: Mutex::new(None),
        }
    }

    /// Run a test on this repo
    pub fn run_test(&self, test: impl FnOnce(&TestContext<Tools>) -> Result<()>) -> Result<()> {
        self.run_test_inner(None, test)
    }

    /// Run a test on a variant of this repo, checked out into a dir with the given name (id).
    ///
    /// Normally tests are ~free to mess with the checkout since they run in serial, but this
    /// is useful if you want multiple modifications of the checkout to exist at once
    /// (e.g. if you're handing all of them to the gallery workspace).
    pub fn run_test_with_id(
        &self,
        id: &str,
        test: impl FnOnce(&TestContext<Tools>) -> Result<()>,
    ) -> Result<()> {
        self.run_test_inner(Some(id), test)
    }

    fn run_test_inner(
        &self,
        id: Option<&str>,
        test: impl FnOnce(&TestContext<Tools>) -> Result<()>,
    ) -> Result<()> {
        let maybe_tools = self.tools.lock();
        // Intentionally unwrapping here to poison the mutexes if we can't fetch
        let tools_guard = Self::init_mutex(maybe_tools, || Tools::default());
        let tools = tools_guard.as_ref().unwrap();

        // If there's an explicit id, then we're trying to do a "variant" test that is independent
        // from the main variant. If so, create a new temporary `Mutex<Option<RawTestContext>>>`
        // so that we don't clobber the "true" variant. It's fine that the result won't be cached
        // properly -- variants are likely to be one-offs so caching is pointless.
        let raw_ctx_lock;
        let maybe_repo = if id.is_some() {
            raw_ctx_lock = Mutex::new(None);
            raw_ctx_lock.lock()
        } else {
            self.ctx.lock()
        };
        let raw_ctx_guard = Self::init_mutex(maybe_repo, || self.init_context(id, tools).unwrap());
        let raw_ctx = raw_ctx_guard.as_ref().unwrap();

        let ctx = TestContext { raw_ctx, tools };
        // Ensure we're in the right dir before running the test
        CommandInfo::set_working_dir(&ctx.working_dir);

        test(&ctx)
    }

    /// Create the RawTestContext for this Repo by git fetching it to a sufficient temp dir
    fn init_context(&self, id: Option<&str>, tools: &Tools) -> Result<RawTestContext> {
        let Repo {
            repo_owner,
            repo_name,
            commit_ref,
            subdir,
            ..
        } = self.repo;
        let repo_url: String = format!("https://github.com/{repo_owner}/{repo_name}");
        let repo_id = id
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("{repo_owner}_{repo_name}_{commit_ref}"));
        let repo_dir = Utf8Path::new(TARGET_TEMP_DIR).join(&repo_id);
        let working_dir = if let Some(subdir) = subdir {
            repo_dir.join(subdir)
        } else {
            repo_dir.clone()
        };

        // Clone the repo we're interested in and cd into it
        Self::fetch_repo(tools.git(), &repo_dir, &repo_url, commit_ref)?;

        // Run tests
        let ctx = RawTestContext {
            repo: self.repo,
            repo_id,
            repo_dir,
            working_dir,
        };
        Ok(ctx)
    }

    /// Take a potentially-poisoned, potentially-unintializeed `MutexGuard<Option<T>>` and
    /// handle the poison and initialization of it.
    ///
    /// It's fine for the mutex to be poisoned once the value is Some because none of the tests
    /// are allowed to mutate the TestContext. But if it's poisoned while None that means we
    /// encountered an error while setting up the TestContext and should just abort everything
    /// instead of retrying over and over. (e.g. git fetch failed, finding tools failed, etc.)
    fn init_mutex<T>(
        maybe_guard: std::sync::LockResult<MutexGuard<'_, Option<T>>>,
        init: impl FnOnce() -> T,
    ) -> MutexGuard<'_, Option<T>> {
        let mut guard = match maybe_guard {
            Ok(guard) => guard,
            Err(poison) => {
                let guard = poison.into_inner();
                if guard.is_none() {
                    panic!("aborting all tests: failed test harness initialization");
                }
                guard
            }
        };

        if guard.is_none() {
            let ctx = init();
            *guard = Some(ctx);
        }
        guard
    }

    /// Fetch/update a repo to the given commit_ref
    fn fetch_repo(
        git: &CommandInfo,
        repo_dir: &Utf8Path,
        repo_url: &str,
        commit_ref: &str,
    ) -> Result<()> {
        if repo_dir.exists() {
            eprintln!("repo already cloned, updating it...");
            CommandInfo::set_working_dir(repo_dir);
            git.output_checked(|c| c.arg("remote").arg("set-url").arg("origin").arg(repo_url))?;
            git.output_checked(|c| c.arg("fetch").arg("origin").arg(commit_ref))?;
            git.output_checked(|c| c.arg("reset").arg("--hard").arg("FETCH_HEAD"))?;
        } else {
            eprintln!("fetching {repo_url}");
            axoasset::LocalAsset::create_dir_all(repo_dir)?;
            CommandInfo::set_working_dir(repo_dir);
            git.output_checked(|c| c.arg("init"))?;
            git.output_checked(|c| c.arg("remote").arg("add").arg("origin").arg(repo_url))?;
            git.output_checked(|c| c.arg("fetch").arg("origin").arg(commit_ref))?;
            git.output_checked(|c| c.arg("reset").arg("--hard").arg("FETCH_HEAD"))?;
        }

        Ok(())
    }
}
