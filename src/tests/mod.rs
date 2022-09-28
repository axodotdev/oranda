use crate as oranda;

#[test]
fn returns_css() {
    let site = oranda::create_site("# hello");
    assert!(site
        .css
        .contains("--text-light:#fafafa;--text-800:#1f2937;"));
}

#[test]
fn parses_basic_markdown() {
    let site = oranda::create_site("# hello");
    assert!(site.html.contains("<h1>hello</h1>"));
}

#[test]
fn parses_images() {
    let site = oranda::create_site("![Stormtroopocat](https://test.com/test.jpg)");
    assert!(site
        .html
        .contains("<img src=\"https://test.com/test.jpg\" alt=\"Stormtroopocat\" />"));
}

#[test]
fn parses_lists() {
    let site = oranda::create_site("- A list item");
    assert!(site.html.contains("<li>A list item</li>"));
}

#[test]
fn parses_links() {
    let site = oranda::create_site("[link text](http://test.com)");
    assert!(site
        .html
        .contains("<a href=\"http://test.com\">link text</a>"));
}

#[test]
fn parses_code() {
    let site = oranda::create_site(
        r#"

```js
var foo = function (bar) {
  return bar++;
};

console.log(foo(5));
```
    "#,
    );
    assert!(site.html.contains("<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">var </span><span style=\"color:#8fa1b3;\">foo </span><span style=\"color:#c0c5ce;\">= </span><span style=\"color:#b48ead;\">function </span><span style=\"color:#c0c5ce;\">(bar) {\n</span><span style=\"color:#c0c5ce;\">  </span><span style=\"color:#b48ead;\">return </span><span style=\"color:#bf616a;\">bar</span><span style=\"color:#c0c5ce;\">++;\n</span><span style=\"color:#c0c5ce;\">};\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">console.</span><span style=\"color:#96b5b4;\">log</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#8fa1b3;\">foo</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">));\n</span></pre>\n</code></pre>"));
}
