# Contributing

Here's some helpful tips for contributing.

## Auto-recompiling based on source changes

If you're working on oranda and you want to rebuild both oranda itself and your preview site when stuff changes,
this is the command to keep in mind (assuming you have `cargo-watch` installed):

```shell
cargo watch -x "run dev"
```

On some platforms, apparently images also get recompiled and picked up by cargo-watch:

```shell
cargo watch -x "run dev" --ignore "*.png"  --ignore "*.jpg"
```

## ...plus working on the CSS

As long as you're working with a development build of oranda (by running `cargo run` without the `--release` flag),
oranda will automatically download a version of the Tailwind compiler and compile the CSS from your
local checkout for you.

`oranda dev` doesn't automatically reload on CSS changes (because it's meant to be used by users),
but you can include the CSS directory manually like such:

```shell
cargo run dev -i oranda-css/css/
```

## Updating syntax highlighting languages

We use [syntect] to support syntax highlighting in Markdown code blocks. If you want to add support for a new language
that's not included in syntect's default set of languages or the ones oranda provides, you'll need to extend the
`oranda::site::markdown::syntax_highlight::dump_syntax_themes` function to load your new `.sublime-syntax` file from disk
and to include it in our syntax set dump. This function, once adjusted, only needs to be ran once manually, by including
it anywhere in the call path of the application (I recommend somewhere close to the top of the build CLI function).

### Converting from .tmLanguage

`syntect` accepts `.sublime-syntax` files, but Sublime Text can also accept `.tmLanguage` (TextMate syntax bundles) files,
so sometimes we need to convert from one to the other. Thankfully, the Sublime Text editor has a built-in feature for this.
Here's what you need to do:

1. Download and install Sublime Text
2. In Sublime Text, from the menu, select Tools -> Developer -> New Syntax...
3. This puts you in your Packages/User folder. Paste your tmLanguage file contents and save as `<yourlang>.tmLanguage`.
4. Next, you should be able to run Tools -> Developer -> New Syntax from <yourlang>.tmLanguage...
5. This opens a new tab with the converted output. Save and copy it or paste it into a new file in oranda. Profit!

[syntect]: https://crates.io/crates/syntect