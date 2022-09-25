use std::fs::{self, File};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let md = fs::read_to_string("../test.md")?;
    let msg = oranda::create_html(&md);
    let mut file = File::create("public/index.html")?;
    file.write_all(msg.as_bytes())?;
    Ok(())
}
