use oranda_generate_css::default_css_output_dir;

fn main() {
    // Build the CSS on-demand if we're running a release-ish build here (as determined by Cargo)
    if !cfg!(debug_assertions) {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .max_blocking_threads(128)
            .enable_all()
            .build()
            .expect("Initializing tokio runtime failed");
        let _guard = runtime.enter();
        oranda_generate_css::build_css(&default_css_output_dir()).unwrap();
    }
}
