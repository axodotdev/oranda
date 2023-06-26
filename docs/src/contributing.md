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

To recompile the CSS on changes, in another Terminal, run:

```shell
yarn install
yarn dev
```

You can then include your locally built CSS in your `cargo-watch` command:

```shell
ORANDA_CSS=oranda-css/dist/oranda.css cargo watch -x "run dev -i oranda-css/dist/oranda.css"
```

## Updating syntax highlighting languages

We use [syntect] to support syntax highlighting in Markdown code blocks. If you want to add support for a new language
that's not included in syntect's default set of languages or the ones oranda provides, you'll need to extend the
`oranda::site::markdown::syntax_highlight::dump_syntax_themes` function to load your new `.sublime-syntax` file from disk
and to include it in our syntax set dump. This function, once adjusted, only needs to be ran once manually, by including
it anywhere in the call path of the application (I recommend somewhere close to the top of the build CLI function).