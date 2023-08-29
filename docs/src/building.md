# Building oranda

In case you're interested in building oranda from scratch, be it for packaging for a software distribution or
something different, here's some information on things to keep in mind!

## Basic setup

To build oranda from source, you'll need the following:

- Stable Rust toolchain
- _Optionally_, if you want to pre-compile CSS, Node & npm
- `cargo-dist`, if you want to opt into the same release flags we use in our CI

oranda can be built from source by running `cargo build --release` (not including the CSS - read below for more).
Since we use `cargo-dist` to build our binaries for the official distribution of oranda, you may want to
run `cargo dist build` instead. This
ensures that you generate the same builds that we do.

### The trouble with CSS

oranda includes some CSS to style pages - we call this oranda-css. This CSS uses [TailwindCSS](https://tailwindcss.com),
and therefore needs to be compiled before it can be included. To figure out _how_ to build and/or include this CSS,
we use a `build.rs` file. This file sets configuration variables for one of these cases:

> **Note**: this is true to the `main` branch and all `0.4.0`-prereleases, but not to 0.3.0. In oranda 0.3.0, the CSS
> will
> always be built using a Tailwind binary, either at build time or at runtime.

- You have the environment variable `ORANDA_USE_TAILWIND_BINARY` set. This causes oranda to attempt to download a
  `tailwindcss` binary, and build using that.
    - If you run this in a development build, the CSS will get built at runtime.
    - If you run it in a release build, the CSS will be built as part of `build.rs` and embedded into the resulting
      binary.
- If a file exists at `oranda-css/dist/oranda.css`, oranda will inline that file and use it as its _current_ version,
  meaning it'll insert it into sites unless the user overrides it with a different version. This means you can prebuild
  the CSS using npm, and then run `cargo build` to let it be picked up.
- If neither of these conditions are true, a Cargo build will produce a binary that'll always fetch the CSS from our
  GitHub releases. Stricly seen, this is a **worse version of oranda** (because it has to do extra CSS requests), so
  we suggest not distributing a binary that was built this way. You can check if a binary was built this way by looking
  out for the following log
  line: `warning: This build of oranda will pull CSS directly from GitHub releases! This is probably not what you want.`

> For `cargo install` users: Your `oranda` binary is of the third type in the list above. This is unfortunately a
> shortcoming of Cargo's build pipeline, but if you're fine with using a slightly slower version of
> oranda, `cargo install`
> works fine. If you want a regular binary, check the [install page](./install.md).

If you're distributing binaries anywhere, you can either use the Node toolchain, or the Tailwind binary
using `ORANDA_USE_TAILWIND_BINARY`, depending on which is easier/more conformant in your build environment.

```sh
# either:
ORANDA_USE_TAILWIND_BINARY=true cargo dist build

# or:
cd oranda-css
npm run build
cd ..
cargo dist build
```

