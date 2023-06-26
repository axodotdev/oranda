# Configuration

- [Configuration](#configuration)
  - [Manifest file: `oranda.json`](#manifest-file-orandajson)
  - [Project Configuration](#project-configuration)
    - [`name`](#name) 📦 - the name of your application
    - [`version`](#version) 📦 - current version of your project
    - [`description`](#description) 📦 - brief description of your project
    - [`homepage`](#homepage) 📦 - url to the homepage of your project
    - [`repository`](#repository) 📦 - url to the repository of your project
    - [`readme_path`](#readmepath) - relative custom path to your project's readme file
    - [`license`](#license) 📦 - license of your project (in SPDX format)
  - [Build Configuration](#build-configuration)
    - [`dist_dir`](#distdir) - path to where built output should be placed
    - [`static_dir`](#staticdir) - path to a directory containing static assets
    - [`path_prefix`](#pathprefix) - a URL fragment to prepend to URLs, useful if hosting from a subfolder
    - [`additional_pages`](#additionalpages) - additional pages to be rendered and linked to
  - [Marketing Configuration](#marketing-configuration)
    - [`analytics`](#analytics) - automatically insert analytics snippets for several providers
    - [`social`](#social) - additional configuration for SEO-related inserts
  - [Theme/Style Configuration](#style-configuration)
    - [`theme`](#theme) - change oranda's CSS theme
    - [`additional_css`](#additionalcss) - additional CSS to insert into your pages
    - [`oranda_css_version`](#orandacssversion) - custom version of oranda's built-in CSS to use
    - [`logo`](#logo) - custom site logo
    - [`favicon`](#favicon) - custom site favicon
  - [Components Configuration](#components-configuration)
    - [`changelog`](#changelog) - extract your changelog from GitHub automatically
    - [`mdbook`](#mdbook-or-mdbook) - let us render a mdbook site for you
    - [`funding`](#funding) - configuration for rendering a site related to project funding methods
    - [`artifacts`](#artifacts) - configuration for displaying downloadable artifacts/installers

> 📦 = automatically collected from your package metadata!

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
  "build": {
    "path_prefix": "oranda"
  },
  "styles": {
    "theme": "axodark",
    "favicon": "https://www.axo.dev/favicon.ico"
  },
  "marketing": {
    "social": {
      "image": "https://www.axo.dev/meta_small.jpeg",
      "image_alt": "axo",
      "twitter_account": "@axodotdev"
    },
    "analytics": {
      "plausible": {
        "domain": "opensource.axo.dev"
      }
    }
  },
  "components": {
    "changelog": true,
    "artifacts": {
      "package_managers": {
        "preferred": {
          "npm": "npm install @axodotdev/oranda --save-dev",
          "cargo": "cargo install oranda --locked --profile=dist"
        },
        "additional": {
          "npx": "npx @axodotdev/oranda",
          "binstall": "cargo binstall oranda",
          "nix-env": "nix-env -i oranda",
          "nix flake": "nix profile install github:axodotdev/oranda"
        }
      }
    }
  }
}
```

> **NOTE:** All paths in `oranda.json` are relative to the `oranda.json` file. We
> recommend placing this file in the same directory as your project manifest, such as a `package.json`
> or `Cargo.toml`.

## Project Configuration

### name

> Added in version 0.1.0.

- Default value: Project manifest `name` field

Your project's name.

### version

> Added in version 0.1.0.

- Default value: Project manifest `version` field.

Your project's current version.

### description

> Added in version 0.1.0.

- Default value: Project manifest `description` field

Your project's description.

### homepage

> Added in version 0.1.0.

- Default value: Project manifest `homepage` field

Your project's homepage.

### repository

> Added in version 0.1.0.

- Default value: Project manifest `repository` field

Your project's Git repository. Right now, only the HTTPS URL works.

### readme_path

> Added in version 0.1.0.

- Default value: A variation of the standard `README.md`

The path to your project's readme file.

### license

> Added in version 0.1.0.

- Default value: Project manifest `license` field.

Your project's license.

## Build Configuration

### dist_dir

> Added in version 0.1.0.

- Default value: `public/`

The directory where your static files will be output to. This must be relative to the `oranda.json` file.

### static_dir

> Added in version 0.1.0.

- Default value: `static/`

Static content that oranda will copy to its output folder. This must be relative to the `oranda.json` file.

### path_prefix

> Added in version 0.1.0.

If you're hosting oranda on a nested path (e.g. `mysite.cool/myproject`), you should set `path_prefix` to
`myproject` in your configuration in order for oranda to generate correct links.

### additional_pages

> Added in version 0.1.0.

An object of additional Markdown pages that you'd like to be included. All of these will appear in the site header.
[More information](./configuration/additional-pages.md)

## Marketing Configuration

### analytics

> Added in version 0.1.0.

[Configuration for page analytics.](./configuration/analytics.md)

### social

> Added in version 0.1.0.

[Options useful for SEO features.](./configuration/social.md)

## Style Configuration

### theme

> Added in version 0.1.0.

- Default value: `dark`

Choose which theme to use. Read more about [themes](./configuration/theme.md).

### additional_css

> Added in version 0.1.0.

Add extra CSS to your pages' header. Read more in the [theme documentation](./configuration/theme.md).

### oranda_css_version

> Added in version 0.1.0.

Specify a version of the embedded oranda CSS. This can be used to opt into newer CSS releases that don't have
an oranda release associated with them yet.

## logo

> Added in version 0.1.0.

Path to a custom logo to be shown in your website header.

### favicon

> Added in version 0.1.0.

Path to a custom favicon.

## Components Configuration

### artifacts

> Added in version 0.1.0.

Configuration for enabling downloadable artifacts, as well as the `cargo-dist`integration.
[More information](./configuration/artifacts.md)

### mdbook or md_book

> Added in version 0.1.0.

[Configuration for mdbook.](./configuration/mdbook.md)

### changelog

> Added in version 0.1.0.

Enable changelog generation. [More information](./configuration/changelog.md)

### funding

> Added in version 0.1.0.

Allows you to tweak or disable oranda's funding page.
[Read more here.](./configuration/funding.md)

## Configuration before 0.1.0

Before version 0.1.0 (the last stable version was/is 0.0.3, the last prerelease was/is 0.1.0-prerelease7), the
configuration format looked like this:

```json
{
  "name": "oranda",
  "description": "generate static sites for your dev tools",
  "dist_dir": "oranda_out",
  "homepage": "https://oranda.axo.dev",
  "static_dir": "static",
  "no_header": false,
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
  },
  "artifacts": {
    "cargo_dist": true
  },
  "logo": "assets/oranda.png",
  "license": "MIT OR Apache-2.0",
  "mdbook": false,
  "path_prefix": "oranda",
  "styles": {
    "theme": "axo_dark"
  },
  "funding": {
    "preferred_funding": "github"
  },
  "changelog": true
}
```
