pub use head::head;

mod head;

pub fn footer() -> &'static str {
    r#"
    </div></div></div></body></html>
    "#
}
