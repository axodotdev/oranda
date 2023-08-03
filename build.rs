use oranda_generate_css::DEFAULT_CSS_OUTPUT_DIR;

fn main() {
    // Build the CSS on-demand if we're running a release-ish build here (as determined by Cargo)
    if std::env::var("PROFILE").is_ok_and(|v| v == "release") {
        oranda_generate_css::build_css(DEFAULT_CSS_OUTPUT_DIR).unwrap();
    }
}
