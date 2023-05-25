# Configuration

`oranda` is designed to work with no configuration- for projects with a
`package.json` or `Cargo.toml`, `oranda` will grab the project metadata it needs
from your project manifest file.

If you project has both a `Cargo.toml` and a `package.json` we recommend defining
project metadata fields like `name` in your `oranda.json`.

## Manifest file: `oranda.json`

If you'd like to customize your project you can do so in a `oranda.json` file.

For example:

```json
{
  "homepage": "https://oranda.axo.dev",
  "readme_path": "dev/README.md",
  "repository": "https://github.com/axodotdev/oranda",
  "additional_pages": {
    "Another page": "dev/additional.md"
  },
  "favicon": "https://www.axo.dev/favicon.ico",
  "analytics": {
    "plausible": {
      "domain": "tools.axo.dev/oranda"
    }
  },
  "social": {
    "image": "https://www.axo.dev/meta_small.jpeg",
    "image_alt": "axo",
    "twitter_account": "@axodotdev"
  }
}
```

> **NOTE:** All paths in `oranda.json` are relative to the `oranda.json` file. We
  recommend placing this file in the same directory as your project manifest, such as a `package.json`
  or `Cargo.toml`.

## Configuration options

Some options have more detailed documentation, indicated by a link from the configuration key.

- `name`: Your project's name. By default fetched from your project's manifest file.
- `description`: Your project's description. By default fetched from your project's manifest file.
- `dist_dir`: The directory where your static files will be output to. By default, this is `public/`.
- `homepage`: Your project's homepage. By default fetched from your project's manifest file.
- `static_dir`: Static content that oranda will copy to its output folder. By default, this is `static/`.
- `no_header`: Whether to hide the page header (title and navigation bar).
- `readme_path`: The path to your project's readme file. oranda by default tries a variation of the standard `README.md`.
- `repository`: Your project's Git repository. By default fetched from your project's manifest file.
- [`analytics`](./configuration/analytics.md): Configuration for page analytics.
- [`additional_pages`](./configuration/additional-pages.md): An object of additional Markdown pages that you'd like to be included. All of these will appear in the site
  header.
- [`social`](./configuration/social.md): Options useful for SEO features.
- [`artifacts`](./configuration/artifacts.md): Configuration for enabling downloadable artifacts, as well as the `cargo-dist`
  integration.
- `version`: Not currently used.
- `logo`: Path to a custom logo to be shown in your website header.
- `favicon`: Path to a custom favicon.
- `path_prefix`: If you're hosting oranda on a nested path (e.g. `mysite.cool/myproject`), you should set `path_prefix` to
  `myproject` in your configuration in order for oranda to generate correct links.
- `license`: Your project's license. By default fetched from your project's manifest file.
- [`mdbook` or `md_book`](./configuration/mdbook.md): Configuration for mdbook.
- [`changelog`](./configuration/changelog.md): Enable changelog generation.
- [`styles`](./configuration/theme.md): Configuration for custom styles or themes.
