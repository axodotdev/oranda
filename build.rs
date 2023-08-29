use oranda_generate_css::default_css_output_dir;
use std::path::PathBuf;

fn main() {
    // Step 1: Has the user set ORANDA_USE_TAILWIND_BINARY? If so, we set a cfg attribute to build
    // the CSS on-demand in the main binary. This is intended to be used by contributors for a faster local
    // development cycle.
    // Alternatively, a packager can set this for a release build to prebuild the CSS using the
    // Tailwind binary.
    if std::env::var("ORANDA_USE_TAILWIND_BINARY").is_ok() || cfg!(feature = "build-with-tailwind")
    {
        if cfg!(debug_assertions) {
            println!("cargo:rustc-cfg=css=\"tailwind\"");
        } else {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .max_blocking_threads(128)
                .enable_all()
                .build()
                .expect("Initializing Tokio runtime failed");
            let _guard = runtime.enter();
            oranda_generate_css::build_css(&default_css_output_dir()).unwrap();
            println!("cargo:rustc-cfg=css=\"file\"");
        }
    } else {
        // Step 2: Does a CSS file exist at oranda-css/dist/oranda.css? If so, assume the user
        // has precompiled oranda CSS, which will cause the main oranda binary to include this
        // file.
        let path = PathBuf::from("./oranda-css/dist/oranda.css");
        if path.exists() {
            println!("cargo:rustc-cfg=css=\"file\"");
        } else {
            // Step 3: The user doesn't have the CSS locally and doesn't want to compile it from
            // scratch. In this case, we let the main binary know that it should always pull CSS
            // from GitHub releases. This behaviour is intended as a fallback for `cargo install`
            // builds.
            println!("cargo:rustc-cfg=css=\"fetch\"");
            println!("cargo:warning=This build of oranda will pull CSS directly from GitHub releases! This is probably not what you want.");
        }
    }
}
