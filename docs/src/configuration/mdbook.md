# mdbook support

oranda can generate [mdbooks][mdbook] for you. If you've already worked with mdbook, it's as simple as pointing oranda
at your book directory using the `mdbook.path` option:

```json
{
  "mdbook": {
    "path": "./docs"
  }
}
```

This will cause oranda to automatically recompile your book for you, which will be served at the `yoursite/book/` URL.
`oranda dev` will also be watching this directory.

## mdbook quickstart

If this is the first time you're working with mdbook, these are the minimal steps you'd need before editing the oranda config.
After you've [installed the mdbook tool][mdbook-install], you can generate a new book scaffold:

```sh
mdbook init docs # replace docs with your preferred directory
```

You can either use `oranda dev` or `mdbook serve docs/` to have a preview for your mdbook.

[mdbook]: https://rust-lang.github.io/mdBook/
[mdbook-install]: https://rust-lang.github.io/mdBook/guide/installation.html
