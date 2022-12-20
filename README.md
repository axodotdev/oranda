# oranda

> beautiful custom websites for your CLI

## Usage

- `build`: generates a static site based on project and/or custom configuration.
  optionally takes a `--path` flag, otherwise looks in the current directory
- `serve`: starts a local server to preview site generated from `build` command.
  uses `dist_dir` to find assets, which defaults to `public`. optionally takes
  a `--port` flag, otherwise uses `7979`.

---

## Rust

```rs
pub fn syntax_highlight(lang: Option<&str>, code: &str) -> Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_from_folder("src/site/markdown/syntax_themes").unwrap();
    let language = match lang {
        None | Some("") => "rs",
        Some(l) => l,
    };

    let syntax = ps.find_syntax_by_extension(language);

}
```

## JavaScript

```js
// Inline Comment
const add = (a, b) => {
  return a + b;
};

let a = 2;
console.log(add(a, 4));
```
