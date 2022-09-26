use oranda;

#[test]
fn parses_basic_markdown() {
    assert!(oranda::create_html("# hello").contains("<h1>hello</h1>"));
}

#[test]
fn parses_images() {
    assert!(
        oranda::create_html("![Stormtroopocat](https://test.com/test.jpg)")
            .contains("<img src=\"https://test.com/test.jpg\" alt=\"Stormtroopocat\" />")
    );
}

#[test]
fn parses_lists() {
    assert!(oranda::create_html("- A list item").contains("<li>A list item</li>"));
}

#[test]
fn parses_links() {
    assert!(oranda::create_html("[link text](http://test.com)")
        .contains("<a href=\"http://test.com\">link text</a>"));
}

#[test]
fn parses_code() {
    assert!(oranda::create_html(
        r#"

```js
var foo = function (bar) {
  return bar++;
};

console.log(foo(5));
```
    "#
    ).contains("<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">var </span><span style=\"color:#8fa1b3;\">foo </span><span style=\"color:#c0c5ce;\">= </span><span style=\"color:#b48ead;\">function </span><span style=\"color:#c0c5ce;\">(bar) {\n</span><span style=\"color:#c0c5ce;\">  </span><span style=\"color:#b48ead;\">return </span><span style=\"color:#bf616a;\">bar</span><span style=\"color:#c0c5ce;\">++;\n</span><span style=\"color:#c0c5ce;\">};\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">console.</span><span style=\"color:#96b5b4;\">log</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#8fa1b3;\">foo</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">));\n</span></pre>\n</code></pre>"));
}
