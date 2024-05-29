# Configuration


**`oranda` is designed to work with no configuration**- for projects with a
`package.json` or `Cargo.toml`, `oranda` will grab the project metadata it needs
from your project manifest file. It can also infer a lot of the things it wants to
render from your already existing environment.

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
        "domain": "axodotdev.github.io"
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

> **NOTE:** All paths in `oranda.json` are relative to the `oranda.json` file.

**See the [configuration reference](./configuration/reference.md) for a detailed explanations of all options!**

## Workspace manifest file: `oranda-workspace.json`

> Added in version 0.3.0.

oranda supports building multiple sites at once (referred to as building in a "workspace"). To control this behavior,
you can create a `oranda-workspace.json` file inside your workspace root. Running an oranda command will pick up this
file, and build the workspace members accordingly.

The workspace file supports all other oranda config keys, which will be passed down to each workspace members.

[Read more about workspaces](configuration/workspaces.md) or [see the workspace reference](./configuration/reference.md#workspace)

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

