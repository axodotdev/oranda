# Install

There's lots of ways to install oranda!

## The Quickest Way

On the [oranda website][website], there's a one-liner command you can execute for your
OS that'll download and install oranda for you, without any further hassle!

## Install Prebuilt Binaries With [cargo-binstall]

```sh
cargo binstall oranda
```

## Build From Source With Cargo

```sh
cargo install oranda --locked --profile=dist
```

> `--profile=dist` is the profile we build our shippable binaries with, it's optional.
>
> `--locked` asks Cargo to respect the lockfile, improving build reproducibility at
> the cost of not getting any bugfixes from newer releases of its dependencies.


## Download Prebuilt Binaries From GitHub Releases

[See the latest release](https://github.com/axodotdev/oranda/releases/latest)!

## Install With NPM

```sh
npm install @axodotdev/oranda
# alternatively:
npx @axodotdev/oranda build
```

## Install With Nix
oranda is available in [`nixpkgs`](https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/misc/oranda/default.nix), and also as a nix flake. This installer is currently experimental, so we don't recommend you use it in production workflows.

On a system with nix installed, you can run
```sh
nix-env -i oranda
```

or to install from GitHub using the flake,
```sh
nix profile install github:axodotdev/oranda
```

[cargo-binstall]:https://github.com/cargo-bins/cargo-binstall
[website]: https://opensource.axo.dev/oranda
