use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.md")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let msg = oranda::create_html(&data);
    let mut file = File::create("public/index.html")?;
    file.write_all(msg.as_bytes())?;
    Ok(())
}
