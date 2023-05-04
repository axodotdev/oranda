# Install

There's lots of ways to install oranda!

## Install Prebuilt Binaries With cargo-binstall

```sh
cargo binstall oranda
```

## Build From Source With Cargo

```sh
cargo install oranda --locked --profile=dist
```

> `--profile=dist` is the profile we build our shippable binaries with, it's optional.
>
> `--locked` asks Cargo to respect the lockfile, improving build reproducibility at the
> the cost of not getting any bugfixes from newer releases of its dependencies.


## Download Prebuilt Binaries From Github Releases

[See The Latest Release](https://github.com/axodotdev/oranda/releases/latest)!


## Use The Installer Scripts

**NOTE: these installer scripts will install to your cargo-home, the same place cargo install does**

Linux and macOS Shell:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/axodotdev/oranda/releases/latest/download/oranda-installer.sh | sh
```

Windows PowerShell:

```sh
irm https://github.com/axodotdev/cargo-dist/releases/latest/download/oranda-installer.ps1 | iex
```

## Install With NPM

```sh
npm install oranda
```