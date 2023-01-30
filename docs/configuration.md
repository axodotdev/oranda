# Configuration

`oranda` is designed to work with no configuration- for projects with a
`package.json` or `Cargo.toml`, `oranda` will grab the project metadata it needs
from your project manifest file.

If you project has both a `Cargo.toml` and a `package.json` we recommend defining
project metadata fields like `name` in your `oranda.json`.

## Manifest file: `oranda.json`

If you'd like to customize your project you can do so in a  `oranda.json` file.

For example:

```json
{
  "homepage": "https://oranda.axo.dev",
  "readme_path": "dev/README.md",
  "theme": "light",
  "repository": "https://github.com/axodotdev/oranda",
  "additional_pages": ["dev/additional.md"],
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

**NOTE: All paths in `oranda.json` are relative to the `oranda.json` file. We
recommend placing this file in the root of your project, alongside any other
project manifest files you may be using, such as a `package.json` or `Cargo.toml`.

## Project Metadata

### Name

The name set will be used in the title of your website and also as the main heading.

By default Oranda will always look for this value in your `package.json`/`cargo.toml` and use it unless the value in the `oranda.json` overrides it.

```json
{
  "name": "String"
}
```

### Description

The description set will be used in the description tag of your website and this is mostly used for SEO.

By default Oranda will always look for this value in your `package.json`/`cargo.toml` and use it unless the value in the `oranda.json` overrides it.

```json
{
  "description": "String"
}
```

### Repository

### Homepage

The homepage set will be used in the meta tags of your website.

By default Oranda will always look for this value in your `package.json`/`cargo.toml` and use it unless the value in the `oranda.json` overrides it.

```json
{
  "homepage": "String"
}
```
