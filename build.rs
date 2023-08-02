fn main() {
    // Build the CSS on-demand if we're running a release-ish build here (as determined by Cargo)
    if std::env::var("PROFILE").is_ok_and(|v| v == "release") {
        oranda_generate_css::build_css("oranda-css/dist/").unwrap();
        println!("cargo:rustc-env=ORANDA_CSS_EXISTS=true");
    }
}
