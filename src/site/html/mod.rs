pub use head::head;
pub use logo::get_logo;

mod head;
mod logo;

pub fn footer() -> &'static str {
    r#"
    </div></div></div></body></html>
    "#
}
