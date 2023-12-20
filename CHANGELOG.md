# Changelog

## 0.6.1 - 2023-12-20

This patch release tweaks behaviour to have oranda prefer its own `curl | sh` expressions
over the ones cargo-dist generates, because it has more context than cargo-dist
to produce the best possible expression.


## 0.6.0 - 2023-12-14

### Features

- **axo Releases support** - [impl](https://github.com/axodotdev/oranda/pull/672)

  oranda now supports axo's homegrown release hosting service, axo Releases! At time of writing,
  axo Releases is still in closed beta, however, if you have access and want to try it out, you can set the following
  configuration parameter:

  ```json
  {
    "components": {
      "source": "axodotdev"
    }
  }
  ```

  Doing so will cause oranda to not fetch from GitHub _at all_, instead using axo Releases as a source of truth for your release data.

- **App names show on multi-app releases** - [impl](https://github.com/axodotdev/oranda/pull/671), issue [#1](https://github.com/axodotdev/oranda/pull/666), [#2](https://github.com/axodotdev/oranda/issues/665) (thanks
  to [aumetra]!)

  If you use oranda to work with a project that publishes multiple differently-named binaries, your install widget may
  end up looking very confusing, with lots of similarly-named tabs. Previously, your options were to set up a
  [workspace][workspace-docs], or to set the [`components.artifacts.match_package_names`][package-names] configuration
  setting. With this release, if you want to keep everything inside a single website, the install widget will now
  show the app names in the tabs, like this:

  ![picture of an app with lots of tabs and app names](https://i.imgur.com/K0NG2SQ.png)

  There's no configuration for this, it'll automatically enable if oranda detects that you have multiple "installable"
  apps in your release assets.

### Bug Fixes

- **Fixed a bug that could cause mdbook compilation to recurse infinitely** - [impl](https://github.com/axodotdev/oranda/pull/674), [issue](https://github.com/axodotdev/oranda/issues/669)

  mdbook makes it very easy to put your book destination directory in the same directory as your book source, which in turn
  causes your built files to be read as source files, over and over again. We detect this now and don't let you do it anymore.

### Housekeeping

- Minor cleanups to the source code, such as finally settling the "who has the better line endings, Linux or
  Windows" debate (it's Linux)

[aumetra]: https://github.com/aumetra
[workspace-docs]: https://opensource.axo.dev/oranda/book/configuration/workspaces.html
[package-names]: https://opensource.axo.dev/oranda/book/configuration/reference.html#componentsartifactsmatch_package_names

## 0.5.0 - 2023-10-10

### Features

- **RSS feed support** - [shadows-withal]/[pr656], [zkat]

  When you have changelog functionality enabled, oranda will now automatically
  generate a RSS feed for it! This means you can plug it into your RSS reader, or
  use further automation that works with RSS files 🙌

  The link to the RSS feed will also be shown on the changelog page. You can disable
  the feed using the new `components.changelog.rss_feed` option.

- **Removed hard dependency on README.md** - [shadows-withal]/[pr640]

  In previous releases, you needed to have a README.md file for oranda to work.
  This is a holdover from the earliest version of oranda (which was literally just
  a tool to generate HTML from your readme files), but with all the since added
  functionality, there's no real reason to keep this requirement. This means that
  **oranda now works without a readme file**!!

- **New `match_package_names` option for workspace config** - [06chaynes]/[pr652]

  When you have a workspace with multiple projects that all publish releases in the same
  GitHub repository, oranda will get confused and think that every release belongs to
  every project. With the new `components.artifacts.match_package_names`, each workspace
  member will now only include releases that contain their project name in the Git tag
  (`v0.4.0-myproject`, for example).

- **Ignore `public/` directory when running `oranda generate ci`** - [shadows-withal]/[pr649], [konstin]/[i643]

  When running the CI generator command, it'll now prompt you whether it should append the `public/`
  directory to your `.gitignore` file.

### Fixes

- **Improved errors for when explicitly declared paths don't exist** - [shadows-withal]/[pr640], [konstin]

  There used to be some cryptic errors oranda would throw if it couldn't find a path you'd manually
  specified in the configuration file - these errors are much nicer now, and their handling is more consistent.

- **Workspace index now supports `additional_css`** - [06chaynes]/[pr653]

  The `additional_css` option will now be respected on the workspace index page!

- **Set correct colors for mdbook draft chapters** - [06chaynes]/[pr655], [hawkw]/[i654]

  In custom oranda themes, mdbook draft chapter titles in the sidebar were unreadable. This has been fixed!

[i643]: https://github.com/axodotdev/oranda/issues/643
[i654]: https://github.com/axodotdev/oranda/issues/654
[pr640]: https://github.com/axodotdev/oranda/pull/640
[pr649]: https://github.com/axodotdev/oranda/pull/649
[pr652]: https://github.com/axodotdev/oranda/pull/652
[pr653]: https://github.com/axodotdev/oranda/pull/653
[pr655]: https://github.com/axodotdev/oranda/pull/655
[pr656]: https://github.com/axodotdev/oranda/pull/656
[shadows-withal]: https://github.com/shadows-withal
[zkat]: https://github.com/zkat
[konstin]: https://github.com/konstin
[06chaynes]: https://github.com/06chaynes
[hawkw]: https://github.com/hawkw

## 0.4.1 - 2023-09-27

### Fixes

- **Fixed MSI checksums getting displayed as separate downloads** - [shadows-withal]/[pr637], [Gankra]/[i636]

  As [cargo-dist 0.3.0](https://github.com/axodotdev/cargo-dist/releases/tag/v0.3.0) supports MSI installers, we noticed
  a small bug in the way oranda tries to infer artifacts, where a checksum for certain types of files can get
  misinterpreted as its own downloadable file. This is now fixed!

[i636]: https://github.com/axodotdev/oranda/issues/636
[pr637]: https://github.com/axodotdev/oranda/pull/637
[shadows-withal]: https://github.com/shadows-withal
[Gankra]: https://github.com/Gankra

## 0.4.0 - 2023-09-11

### Features

- **Improved CSS generation** - [shadows-withal]/[pr617], [ashleygwilliams]/[i610]

  Users and distributors now have more control over how oranda and especially its CSS gets built! You can choose to
  opt-in to building CSS at build or runtime using the Tailwind binary, or you can explicitly prebuild the CSS using
  npm. For more information, see the [docs](https://opensource.axo.dev/oranda/book/building.html#the-trouble-with-css).

- **New `generate` command** - [shadows-withal]/[pr621], [ashleygwilliams]/[i188]

  You can now generate a GitHub Actions CI file that'll build your site and deploy it to GitHub Pages by running
  `oranda generate ci`, which runs you through an interactive wizard! In the future, we'll add more types of
  generatable files.

- **LiveReload support for `oranda dev`** - [shadows-withal]/[pr622], [jamesmunns]/[i556]

  A pretty simple change that has a lot of quality-of-life: Your `oranda dev` browser tab now reloads automatically
  when you change a relevant file and oranda rebuilds your site!

- **Use Axo's favicon by default** - [shadows-withal]/[pr628], [ashleygwilliams]/[i110]

  Oranda sites that don't have a custom favicon now fall back to Axo's favicon, just so that the space in the tab
  bar isn't so drab and lonely.

- **`oranda` is now available on Homebrew!** - [shadows-withal]/[pr602], the cargo dist team <3

  Run `brew install axodotdev/tap/oranda` to install it!

### Fixes

- **Fixed Tailwind breaking the Nix flake** - [hawkw]/[pr609]

  Because Tailwind was being downloaded on each build in a global cache directory (which Nix doesn't support),
  @hawkw added the Tailwind binary as an explicit dependency, and enhanced our Tailwind build code with the
  ability to use a preexisting Tailwind binary.
- **Improved workspace member precedence handling** - [shadows-withal]/[pr626], [shadows-withal]/[i615]

  There were a couple of tough corner cases when using workspace functionality that would incorrectly cascade to the
  members, which is fixed now!
- **Hide changelog sidebar on mobile** - [shadows-withal]/[pr614], [zkat]/[i612]
- **Fixed mdbook text highlighting colors having too little contrast** - [shadows-withal]/[pr627], [Gankra]/[i578]

### Housekeeping

- Factored out a bunch of code into `axoproject` - [mistydemeo]/[pr590]
- Updated `chrono` to drop a very old `time` version - [mistydemeo]/[pr632]

[i110]: https://github.com/axodotdev/oranda/issues/110
[i188]: https://github.com/axodotdev/oranda/issues/188
[i556]: https://github.com/axodotdev/oranda/issues/556
[i578]: https://github.com/axodotdev/oranda/issues/578
[i610]: https://github.com/axodotdev/oranda/issues/610
[i612]: https://github.com/axodotdev/oranda/issues/612
[i615]: https://github.com/axodotdev/oranda/issues/615
[pr590]: https://github.com/axodotdev/oranda/pull/590
[pr602]: https://github.com/axodotdev/oranda/pull/602
[pr609]: https://github.com/axodotdev/oranda/pull/609
[pr614]: https://github.com/axodotdev/oranda/pull/614
[pr617]: https://github.com/axodotdev/oranda/pull/617
[pr621]: https://github.com/axodotdev/oranda/pull/621
[pr622]: https://github.com/axodotdev/oranda/pull/622
[pr626]: https://github.com/axodotdev/oranda/pull/626
[pr627]: https://github.com/axodotdev/oranda/pull/627
[pr628]: https://github.com/axodotdev/oranda/pull/628
[pr632]: https://github.com/axodotdev/oranda/pull/632
[mistydemeo]: https://github.com/mistydemeo
[hawkw]: https://github.com/hawkw
[zkat]: https://github.com/zkat
[ashleygwilliams]: https://github.com/ashleygwilliams
[shadows-withal]: https://github.com/shadows-withal
[jamesmunns]: https://github.com/jamesmunns
[Gankra]: https://github.com/Gankra

## 0.3.1 - 2023-08-18

### Fixes

- Updated the published version of the README on crates.io and npm.
- Our integration tests now use fixtures that have frozen GitHub releases, so that test data doesn't unexpectedly change.

## 0.3.0 - 2023-08-17

### BREAKING CHANGES

- **Changelog autodetect** - [shadows-withal]/[pr583]

  oranda's [changelog feature][changelog-docs] is now opt-out instead of opt-in, meaning that even if you haven't set
  `components.changelog = true`, oranda will now generate a (maybe empty) changelog page. You can opt out of this by
  setting `components.changelog = false`.

  Additionally, oranda will now attempt to read changelog information from a local `CHANGELOG(.md)` or `RELEASES(.md)`
  file, should it exist. If you want oranda to use GitHub release bodies instead, set
  `components.changelog.read_changelog_file = false`.

  **Migration instructions from 0.2.0 to 0.3.0**

  Set `components.changelog` to `false` in your `oranda.json` file if you previously didn't use the changelog feature.
  Also, oranda will now attempt to read changelog information from your local `CHANGELOG.md`/`RELEASES.md` file. If you want
  oranda to use GitHub release bodies instead, set `components.changelog.read_changelog_file = false`.


### Features

- **Workspace Support** - [shadows-withal]/many PRs, [mistydemeo]/many PRs, [jamesmunns]/[i493]

  You can now tell oranda to build sites from multiple projects in the same directory at the same time! By default, this will also generate a separate "root"
  page, providing an index into all projects defined within your workspace.

  To enable this feature, create a new file called `oranda-workspace.json` in your workspace root. This file
  can contain all regular oranda configuration, which will in turn be passed down to each of its children, but it
  also listens to the `workspace` key, which is where you properly configure your workspace. A sample workspace configuration
  would look like this:

  ```json
  {
    "workspace": {
      "auto": true,
      "docs_path": "README.md"
    },
    "styles": {
      "theme": "hacker"
    }
  }
  ```

  This configuration will attempt to auto-detect a Cargo or npm workspace, set the Hacker theme for all members, and
  embed the contents of the top-level `README.md` file into the workspace index page.

  You can also specify workspace members manually, like this:

  ```json
  {
    "workspace": {
      "members": [
        {
          "slug": "memberone",
          "path": "./member-one"
        },
        {
          "slug": "membertwo",
          "path": "./member-two"
        }
      ]
    }
  }
  ```

  > Aside: If you have a use-case for an oranda page that combines multiple different projects _outside_ of a shared directory,
    please let us know on [Discord][discord]!

  For more information on workspace members, [take a look at the docs][workspace-docs]!

- **Inlining CSS** - [shadows-withal]/[pr565], [pr566], [i554]

  oranda now uses a CSS version that's included in the binary it's shipped with! This means no more HTTP requests to GitHub
  to fetch a CSS version multiple times per build. As a bonus, we removed the internal dependency on a Node.js toolchain to build
  the CSS in development, which should make hacking on oranda and its themes a lot easier!

- **Basic CSS caching** - [jamesmunns]/[pr551]

  In line with workspace support, oranda will now attempt to keep already downloaded versions of its CSS in-memory, which
  helps tremendously when you have a lot of workspace members all using a custom CSS version.

- **Artifacts JSON output** - [shadows-withal]/[pr589]

  oranda will now spit out an extra `artifacts.json` file that contains most of the info we use to construct our artifacts page.
  The intention is for this file to be used for external integrations, since JSON is a lot easier to read.
  We've also added a new `--json-only` flag to the build command that only outputs this JSON file.

### Fixes

- **Display platforms alphabetically in install widget** - [Plecra]/[pr544], [shadows-withal]/[i480]

  Platforms are now sorted alphabetically in the install widget dropdown. This is an improvement over the
  previous unsorted state.

- **Show prerelease contents on changelog pages** - [shadows-withal]/[pr549]

  This is a simple bug fix. Previously, we accidentally hid the body of a prerelease on its own separate changelog page
  (but mysteriously, it showed up on the main changelog page when prereleases were toggled!)

- **Restrict parsed repo URLs to GitHub only** - [Plecra]/[pr553]

  Right now, we only support GitHub repository URLs to get context from. This fixed an issue where technically, oranda
  would attempt to do this with GitLab URLs as well, which would cause unintended behavior.

- **Support `git+https` URLs** - [shadows-withal]/[pr563], [geelen]/[i531]

  oranda now correctly handles `git+https://yourrepo` repository URLs, and is a lot more informative when it encounters
  one that it _can't_ parse.

- **Re-add logo class to logo `img` element** - [shadows-withal]/[pr585], [tertsdiepraam]/[i582]

### Housekeeping

- **Testing rework** - [Gankra]/[pr575], [shadows-withal]/[pr581]

  oranda's tests have long been suboptimal, but we now sport a pretty good test suite, with automated integration
  snapshot testing for multiple external projects, and improved HTML-aware integration tests.

[i480]: https://github.com/axodotdev/oranda/issues/480
[i493]: https://github.com/axodotdev/oranda/issues/493
[i531]: https://github.com/axodotdev/oranda/issues/531
[i554]: https://github.com/axodotdev/oranda/issues/554
[i582]: https://github.com/axodotdev/oranda/issues/582
[pr532]: https://github.com/axodotdev/oranda/pull/532
[pr544]: https://github.com/axodotdev/oranda/pull/544
[pr549]: https://github.com/axodotdev/oranda/pull/549
[pr551]: https://github.com/axodotdev/oranda/pull/551
[pr553]: https://github.com/axodotdev/oranda/pull/553
[pr563]: https://github.com/axodotdev/oranda/pull/563
[pr565]: https://github.com/axodotdev/oranda/pull/565
[pr566]: https://github.com/axodotdev/oranda/pull/566
[pr575]: https://github.com/axodotdev/oranda/pull/575
[pr581]: https://github.com/axodotdev/oranda/pull/581
[pr583]: https://github.com/axodotdev/oranda/pull/583
[pr585]: https://github.com/axodotdev/oranda/pull/585
[pr589]: https://github.com/axodotdev/oranda/pull/589
[shadows-withal]: https://github.com/shadows-withal
[Plecra]: https://github.com/Plecra
[jamesmunns]: https://github.com/jamesmunns
[geelen]: https://github.com/geelen
[mistydemeo]: https://github.com/mistydemeo
[tertsdiepraam]: https://github.com/tertsdiepraam

[workspace-docs]: https://opensource.axo.dev/oranda/book/configuration/workspaces.html
[changelog-docs]: https://opensource.axo.dev/oranda/book/configuration/changelog.html
[discord]: https://discord.gg/8BwyXQmeUT

## 0.2.0 - 2023-07-19

### BREAKING

- **make artifact autodetect configurable - [Gankra]/[pr527]**

  We now provide a new boolean key, `components.artifacts.auto`, that lets you explicitly
  enable autodetection of artifacts. Previously, we would only enable this if you either
  turned on `components.artifacts.cargo_dist`, or if you provided some package manager entries.
  Since oranda _does_ also support gleaning artifacts even without `cargo-dist` support enabled,
  we added this extra switch to let you toggle it without having to mess around with package managers.

  This is a **breaking change**, as enabling `cargo-dist` support or specifying package managers does
  not turn on auto-detection of artifacts anymore. If you were previously relying on auto-detection, your
  artifacts will no longer be displayed. To re-enable auto-detection, create a `oranda.json` file if you don't
  already have one, and set the following configuration:

  ```json
  {
    "components": {
      "artifacts": {
        "auto": true
      }
    }
  }
  ```

### Features

- **Typescript syntax highlighting support - [shadows-withal]/[pr525], [geelen]/[i513]**

  You can now use the `ts`/`typescript` languages in code blocks! Hooray for types!

- **Better logo positioning - [SaraVieira]/[pr524], [tertsdiepraam]/[i519]**

  Logos set via the `styles.logo` option will now be properly centered/aligned in all themes,
  and set to a maximum width so that a 1920x1080 logo won't be displayed in its full width and height, thus
  pushing all content down below the fold.

### Fixes

- **Hacker theme highlight color - [SaraVieira], [pr523]/[i522]**

  Selecting text in the Hacker theme now applies a nice, green, high contrast highlight background
  color, instead of being the same color as the text, therefore hiding the content.

- **Package managers documentation - [shadows-withal]/[pr521], [tertsdiepraam]/[i520]**

  Some minor fixes to bring the package manager docs up to speed with how oranda actually processes options.

### Maintenance

- **Refactor into using minijinja templates instead of axohtml - [shadows-withal]/[pr526]**

  A biiig internal refactor moving us away from our previous typed-HTML-in-Rust approach of generating
  HTML, towards using a proper template language (Jinja2) instead. This allows for a lot more flexibility
  and separation of concerns going forward!

[i513]: https://github.com/axodotdev/oranda/issues/513
[i519]: https://github.com/axodotdev/oranda/issues/519
[i520]: https://github.com/axodotdev/oranda/issues/520
[i522]: https://github.com/axodotdev/oranda/issues/522
[pr521]: https://github.com/axodotdev/oranda/pull/521
[pr523]: https://github.com/axodotdev/oranda/pull/523
[pr524]: https://github.com/axodotdev/oranda/pull/524
[pr525]: https://github.com/axodotdev/oranda/pull/525
[pr526]: https://github.com/axodotdev/oranda/pull/526
[pr527]: https://github.com/axodotdev/oranda/pull/527
[shadows-withal]: https://github.com/shadows-withal
[geelen]: https://github.com/geelen
[SaraVieira]: https://github.com/SaraVieira
[tertsdiepraam]: https://github.com/tertsdiepraam
[Gankra]: https://github.com/Gankra


## 0.1.1 - 2023-07-04

### Fixes

- **Remove OpenSSL dependency - [ashleygwilliams]/[pr515], [geelen]/[i514]**

    If a release happens without an OpenSSL issue, does it really happen? In trying
    to run `oranda build` on a beta image for Cloudflare Pages, an end user discovered
    that we hadn't fully vanquished our dependency on OpenSSL. With this PR, we've
    made 100% sure we have.

- **Allow schema key in `oranda.json` - [Gankra], [pr506]**

    To improve the user experience of configuring oranda, we error on unexpected
    keys in the `oranda.json` file- which will help people see typos and other
    mistakes. However, this checking was over-eagerly erroring when folks added
    a schema key so they could use VS Code's schema support. This PR makes an
    exception for users including the "non-functional" (in oranda) schema key.

- **Add fallback to macOS Intel artifacts if Apple Silicon detected, but no artifacts found - [Gankra], [pr511]**

    Platform support and detection is slightly more complicated on Apple/macOS
    machines because Apple offers Rosetta2 which allows you to run binaries built
    for older Intel-based systems on the new Apple Silicon ones (but not vice versa).
    This PR updates the install widget's behavior to show artifacts built for
    Apple Intel-based systems if it detects an Apple Silicon system but cannot find
    any binaries built for Apple Silicon.

- **Artifact table width on mobile - [SaraVieira], [pr505]**

    On mobile, the artifact table's width was forcing a scroll. We've updated the
    CSS to fix this!

[pr505]: https://github.com/axodotdev/oranda/pull/505
[pr506]: https://github.com/axodotdev/oranda/pull/506
[pr511]: https://github.com/axodotdev/oranda/pull/511
[i514]: https://github.com/axodotdev/oranda/issues/514
[pr515]: https://github.com/axodotdev/oranda/pull/515
[ashleygwilliams]: https://github.com/ashleygwilliams
[Gankra]: https://github.com/Gankra
[geelen]: https://github.com/geelen
[SaraVieira]: https://github.com/SaraVieira

## 0.1.0 - 2023-07-03

### What is oranda?

oranda is a hands-off static site generator for people who want a website for their tool but don't want to get knee-deep into web development. As long as you have a `README.md` in your directory, you can benefit from oranda. oranda will also try to automatically work with:

- Release artifacts (currently only for GitHub releases)
    - `cargo-dist`-generated ones, and
    - arbitrary release artifacts
- `mdbook`docs
- GitHub-supported project/maintainer funding sources

oranda is designed to _just work_ in a lot of cases, and in cases where it doesn't, it should provide fine-grained configuration so you can make it work for your use case.

### Features

#### Components

##### Github release artifacts inference

We can now not only figure out whether you're using `cargo-dist` _automatically_, but we also try and support arbitrary tarballs, as long as they're attached to a release and they're following the target-triple format. Oranda will now, for example, pick up a release artifact called `myapp-aarch64-apple-darwin.tar.xz`, even if the project isn't using `cargo-dist` to publish releases.

##### Smarter install widget

The installer widget on our main page has been upgraded! It now not only shows a select box where you can switch between different architectures (though we still attempt to figure out what platform you're running on), but it now additionally displays your package managers, all in a sleek tab-style view:

![oranda install widget preview](https://github-production-user-asset-6210df.s3.amazonaws.com/6445316/250566861-a635c28b-d4d3-4c90-a685-8e9a85673651.png)

You can customize which package managers you want to be displayed on in this widget vs. which ones should only be displayed on the separate install page, as well. [Read more in the docs](https://opensource.axo.dev/oranda/book/configuration/artifacts.html#adding-package-manager-installation-instructions)

##### Funding page

oranda now has the ability to autodetect whether you're using GitHub's funding embed functionality (meaning you have a `.github/FUNDING.yml`), in which case it'll automatically generate a page showing your available funding options.

Additionally, you can enhance this page by selecting a particular funding channel to be prioritized as your "main" funding method. You can also provide custom content from a `funding.md` Markdown file, to provide additional context to your funding page. [Read more in the docs](https://opensource.axo.dev/oranda/book/configuration/funding.html)

This is less of a defined feature and more of an experiment on how we can better integrate maintainers' funding sources onto their websites. Please let us know what you think, or if you have any other feedback or input!

##### `mdbook` autodetect and styling

oranda themes now get applied to your mdbook output, too, meaning there's less of a discrepancy between your flashy oranda page and your default-mdbook-styled docs. We've also been hard at work being able to detect when you use `mdbook` without you having to set it, which should now work in the majority of cases.

#### Configuration

##### New configuration structure

We've completely revamped our configuration structure to be more future-proof and to require less refactoring in the fullness of time. For the new configuration layout, please [consult the docs](https://opensource.axo.dev/oranda/book/configuration.html).

One other major change is that we now **reject unknown config keys**. This means that if you've had a oranda 0.0.3 site, it will now force you to migrate your config to the new format. We've decided on this because we believe that doing anything but hard erroring in this situation would lead to unwanted behavior (old keys getting interpreted in new, weird ways, and so on).

##### Config schema

oranda's configuration schema is now available in a JSON schema for each release starting with this one. This means that in editors like VSCode, you can attach the schema to your `oranda.json` file and get autofill and documentation, like this:

```json
{
  "$schema": "https://github.com/axodotdev/oranda/releases/download/v0.1.0/oranda-config-schema.json"
}
```

#### CLI

##### `dev` command

This release introduces `oranda dev`, which bundles both building your site and serving it from a file server, as well as sprinkling in auto-recompilation when files change. It's intended to be the prime command for local development of a oranda site.

### Bug fixes

(this is a selection, there's been way too many to fully list, at least until we're able to automatically generate a list)

- Various style fixes, lists now display correct, colors should be less offensive to the eye, that sort of stuff
- We're much better now at handling complex release histories!
- We've completely removed the `version` key from the configuration. It wasn't used, and we probably won't use it in the future, either

### Docs

Documentation has had a major rewrite! We now provide a full configuration overview, as well as more detailed writeups for major parts of functionality.

### Thank you to our contributors!

Despite being formally unannounced, several intrepid folks discovered oranda, and have been using it for personal projects and contributing issues and PRs. Their feedback has been invaluable in getting oranda to 0.1.0 today and we'd like to thank them:

- [2mill](https://github.com/2mill)
- [andrewmd5](https://github.com/andrewmd5)
- [jamesmunns](https://github.com/jamesmunns)
- [MarcoIeni](https://github.com/MarcoIeni)
- [msjarvis](https://github.com/msfjarvis)
- [pomdtr](https://github.com/pomdtr)
- [proofconstruction](https://github.com/proofconstruction)
- [tshepang](https://github.com/tshepang)
- [untitaker](https://github.com/untitaker)
- [zkat](https://github.com/zkat)

## 0.0.3 - 2023-05-08

### Features

- **Individual Changelog pages: [shadows-withal]/[↬284]**

    When announcing a new release- it's often desirable to link to an individual
    release page that contains the changelog/release notes. Previously, we built
    a single page for all the releases- now we build individual pages as well.

    This is the first shipped feature from our new team member, Liv! Yay and
    welcome :)

- **npm installer: [ashleygwilliams]/[↬288]**

    As of 0.0.6, `cargo-dist` will build an npm installer for you! So now you
    can npm or npx oranda!

### Fixes

- **Improved configuration support for non-cargo dist users: [pomdtr]/[#262] , [ashleygwilliams]/[↬281]**

    Previously, setting `cargo-dist` as false, or omitting it should have been
    sufficient to stop oranda from attempting to parse your releases as
    cargo-dist artifacts, however `cargo-dist: false` did not work! How the
    entire artifacts config object is handled has been reworked and tested.
    Additionally, work from a refactor has also made projects with a mixture of
    cargo-dist and non-cargo-dist releases work much better.

- **Installer header fallback when platform is not detected: [gankra]/[↬269]**

    When you use cargo-dist, we display a header on your index page that
    detects your user's platform and recommends and installer to them. We did
    not previously have a fallback if we detected a system that none of the
    installers supported. Now, in that scenario- we'll offer an artifact
    download as the header option.

- **Dev commands has proper default values: [ashleygwilliams]/[#256],[↬260]**

    Due to a false hope that `#[derive(Default)]` would collect defaults from
    the `clap` derive API, we shipped the `dev` command with each argument's
    *type's* defaults, not the oranda ones. `dev` now has the same defaults
    that `serve` does, as is to be expected.

- **Improved typography and layout styles on mobile: [ashleygwilliams]/[#234], [SaraVieira]/[↬276]**

### Maintenance

- **Re-add mdbook: [ashleygwilliams]/[#190], [gankra]/[↬285]**
- **Data fetching refactor: [ashleygwilliams]/[#226],[↬274]**
- **Update actions/checkout to v3: [ashleygwilliams]/[#251],[↬255]**

[#190]: https://github.com/axodotdev/oranda/issues/190
[#209]: https://github.com/axodotdev/oranda/issues/209
[#226]: https://github.com/axodotdev/oranda/issues/226
[#234]: https://github.com/axodotdev/oranda/issues/234
[↬240]: https://github.com/axodotdev/oranda/pull/240
[#251]: https://github.com/axodotdev/oranda/issues/251
[↬255]: https://github.com/axodotdev/oranda/pull/255
[#256]: https://github.com/axodotdev/oranda/issues/256
[↬260]: https://github.com/axodotdev/oranda/pull/260
[#262]: https://github.com/axodotdev/oranda/issues/262
[↬269]: https://github.com/axodotdev/oranda/pull/269
[↬274]: https://github.com/axodotdev/oranda/pull/274
[↬276]: https://github.com/axodotdev/oranda/pull/276
[↬281]: https://github.com/axodotdev/oranda/pull/281
[↬284]: https://github.com/axodotdev/oranda/pull/284
[↬285]: https://github.com/axodotdev/oranda/pull/285
[↬288]: https://github.com/axodotdev/oranda/pull/288

[ashleygwilliams]: https://github.com/ashleygwilliams
[pomdtr]: https://github.com/pomdtr
[SaraVieira]: https://github.com/SaraVieira
[gankra]: https://github.com/gankra
[shadows-withal]: https://github.com/shadows-withal

## 0.0.2 - 2023-04-13

### Features

- **Changelogs - [ashleygwilliams]/[#192], [SaraVieira]/[↬193]**

- **`oranda dev` command v1: [ashleygwilliams]/[#209],[↬240]**

    It is common to run `oranda build && oranda serve` so `oranda dev` now wraps
    that into a single function. A future version will improve this to include
    watching to automatically rebuild on changes, while serve continues to run
    uninterrupted.`

- **Additional pages takes a hashmap to allow custom nav names: [ashleygwilliams]/[#115], [SaraVieira]/[↬203]**

    Previously, the nav element for an additional page was inferred from its filename.
    However, this led to some unfortunate presentations- especially if the additional
    page had an all caps filename while others did not. This new data structure
    allows you to provide the specific string you'd like to see in the nav for
    any additional page.

- **Improved error handling: [spitfire05]/[#206], [ashleygwilliams]/[↬191], [SaraVieira]/[↬193],[↬201]**

    Several PRs over the course of the last milestone have dramatically improved
    error handling: from replacing panics with proper errors to adding more
    descriptive errors with messages that help you better understand and solve
    the issue.

- **Themes: [SaraVieira]/[↬208]**

    There are now 2 additional themes- light(default), dark, hacker, and cupcake.

- **Version number and publish date on installer header: [ashleygwilliams]/[#194], [SaraVieira]/[↬217]**

- **Improved styling: [SaraVieira]/[↬202],[↬220]**

### Fixes

- **Code block with unsupported highlight annotations should show as plaintext: [spitfire05]/[#236], [SaraVieira]/[↬237]**

    Annotating a code block with an unsupported language highlight led to blocks
    simply not rendering at all. Now they will render as plainttext and will show
    a warning to request support.

- **Nav should show if using dist/changelog, but not additional pages: [ashleygwilliams]/[#183],[↬187]**

    Originally, we would only generate a nav if a user configured additional
    markdown pages. However, nav is also needed if a user configures their
    project to use cargo-dist or render changelogs.

### Maintenance

- **Update cargo-dist to enable Mac ARM builds: [pomdtr]/[#198], [ashleygwilliams]/[↬248]**

[ashleygwilliams]: https://github.com/ashleygwilliams
[pomdtr]: https://github.com/pomdtr
[SaraVieira]: https://github.com/SaraVieira
[spitfire05]: https://github.com/spitfire05

[#115]: https://github.com/axodotdev/oranda/issues/115
[#183]: https://github.com/axodotdev/oranda/issues/183
[#192]: https://github.com/axodotdev/oranda/issues/192
[#194]: https://github.com/axodotdev/oranda/issues/194
[#198]: https://github.com/axodotdev/oranda/issues/198
[#206]: https://github.com/axodotdev/oranda/issues/206
[#209]: https://github.com/axodotdev/oranda/issues/209
[#236]: https://github.com/axodotdev/oranda/issues/236

[↬187]: https://github.com/axodotdev/oranda/pull/187
[↬191]: https://github.com/axodotdev/oranda/pull/191
[↬193]: https://github.com/axodotdev/oranda/pull/193
[↬201]: https://github.com/axodotdev/oranda/pull/201
[↬202]: https://github.com/axodotdev/oranda/pull/202
[↬203]: https://github.com/axodotdev/oranda/pull/203
[↬208]: https://github.com/axodotdev/oranda/pull/208
[↬217]: https://github.com/axodotdev/oranda/pull/217
[↬220]: https://github.com/axodotdev/oranda/pull/220
[↬237]: https://github.com/axodotdev/oranda/pull/237
[↬240]: https://github.com/axodotdev/oranda/pull/240
[↬248]: https://github.com/axodotdev/oranda/pull/248


## 0.0.1 - 2023-02-24

Initial release.
