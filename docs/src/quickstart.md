# Quickstart

After you've [installed](./install.md) oranda, it's time to give it a spin. Make sure you can execute the
`oranda` command, its output should look something like this:

```
$ oranda
🎁 generate beautiful landing pages for your projects

Usage: oranda [OPTIONS] <COMMAND>

Commands:
  build     Build an oranda site
  dev       Start a local development server that recompiles your oranda site if a file changes
  serve     Start a file server to access your oranda site in a browser
  generate  Generate infrastructure files for oranda sites
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

GLOBAL OPTIONS:
  -v, --verbose
          Whether to output more detailed debug information

      --output-format <OUTPUT_FORMAT>
          The format of the output
          
          [default: human]

          Possible values:
          - human: Human-readable output
          - json:  Machine-readable JSON output
```

Since `oranda` is designed to work without configuration, the quickest start is to just run `oranda dev` in an
existing project with at least a `README.md` file! This will spawn a web server that serves your site, plus
an extra process that watches for changes in files relevant to `oranda`'s build process.

## In a Cargo project

`oranda` integrates with Cargo projects seamlessly. `oranda build` will pick up relevant
metadata from your `Cargo.toml` file automatically, including [`cargo-dist`] configuration,
if you have that set up.

## In a Node project

If you use Node.js, oranda can not only be installed via npm, but also supports reading metadata
from your package manifest file. Additionally, npm scripts make it easy to integrate `oranda` into
your workflows, for example like this:

```json
{
  "scripts": {
    "build:site": "oranda build"
  },
  "dependencies": {
    "@axodotdev/oranda": "~0.3.0"
  }
}
```

## Further Steps

- Explore the [`oranda` configuration options](./configuration.md)
- [Host your site](./hosting.md), on GitHub Pages or elsewhere
- Read the [CLI docs](./cli.md)
- Learn more about [hosting `oranda` sites](./hosting.md)

[`cargo-dist`]: https://opensource.axo.dev/cargo-dist
