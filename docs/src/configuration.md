# Configuration

- [Configuration](#configuration)
  - [Manifest file: `oranda.json`](#manifest-file-orandajson)
  - [Configuration options](#configuration-options)
    - [name](#name)
    - [description](#description)
    - [dist\_dir](#dist_dir)
    - [homepage](#homepage)
    - [static\_dir](#static_dir)
    - [no\_header](#no_header)
    - [readme\_path](#readme_path)
    - [repository](#repository)
    - [analytics](#analytics)
    - [additional\_pages](#additional_pages)
    - [social](#social)
    - [artifacts](#artifacts)
    - [version](#version)
    - [logo](#logo)
    - [favicon](#favicon)
    - [path\_prefix](#path_prefix)
    - [license](#license)
    - [mdbook or md\_book](#mdbook-or-md_book)
    - [changelog](#changelog)
    - [styles](#styles)


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

### name

- Default value: Project manifest `name` field

Your project's name.

### description

- Default value: Project manifest `description` field

Your project's description.

### dist_dir

- Default value: `public/`

The directory where your static files will be output to. This must be relative to the `oranda.json` file.

### homepage

- Default value: Project manifest `homepage` field

Your project's homepage.

### static_dir

- Default value: `static/`

Static content that oranda will copy to its output folder. This must be relative to the `oranda.json` file.

### no_header

> **Note:** This option might change or get removed in the future.

- Default value: `false`

Whether to hide the page header (title and navigation bar).

### readme_path

- Default value: A variation of the standard `README.md`

The path to your project's readme file.

### repository

- Default value: Project manifest `repository` field

Your project's Git repository.

### analytics

[Configuration for page analytics.](./configuration/analytics.md)

### additional_pages

An object of additional Markdown pages that you'd like to be included. All of these will appear in the site header.
[More information](./configuration/additional-pages.md)

### social

[Options useful for SEO features.](./configuration/social.md)

### artifacts

Configuration for enabling downloadable artifacts, as well as the `cargo-dist`integration.
[More information](./configuration/artifacts.md)

### version

> **Note:** This option might change or get removed in the future.

Not currently used.

### logo

Path to a custom logo to be shown in your website header.

### favicon

Path to a custom favicon.

### path_prefix

If you're hosting oranda on a nested path (e.g. `mysite.cool/myproject`), you should set `path_prefix` to
`myproject` in your configuration in order for oranda to generate correct links.

### license

- Default value: Project manifest `license` field.

Your project's license.

### mdbook or md_book

[Configuration for mdbook.](./configuration/mdbook.md)

### changelog

Enable changelog generation. [More information](./configuration/changelog.md)

### styles

[Configuration for custom styles or themes.](./configuration/theme.md)
