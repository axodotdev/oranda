oclif-hello-world
=================

oclif example Hello World CLI

[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![CircleCI](https://circleci.com/gh/oclif/hello-world/tree/main.svg?style=shield)](https://circleci.com/gh/oclif/hello-world/tree/main)
[![GitHub license](https://img.shields.io/github/license/oclif/hello-world)](https://github.com/oclif/hello-world/blob/main/LICENSE)

<!-- toc -->
* [Usage](#usage)
* [Commands](#commands)
<!-- tocstop -->
# Usage
<!-- usage -->
```sh-session
$ npm install -g oclif-generate
$ oclif-generate COMMAND
running command...
$ oclif-generate (--version)
oclif-generate/0.0.0 darwin-arm64 node-v18.16.0
$ oclif-generate --help [COMMAND]
USAGE
  $ oclif-generate COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`oclif-generate hello PERSON`](#oclif-generate-hello-person)
* [`oclif-generate hello world`](#oclif-generate-hello-world)
* [`oclif-generate help [COMMANDS]`](#oclif-generate-help-commands)
* [`oclif-generate plugins`](#oclif-generate-plugins)
* [`oclif-generate plugins:install PLUGIN...`](#oclif-generate-pluginsinstall-plugin)
* [`oclif-generate plugins:inspect PLUGIN...`](#oclif-generate-pluginsinspect-plugin)
* [`oclif-generate plugins:install PLUGIN...`](#oclif-generate-pluginsinstall-plugin-1)
* [`oclif-generate plugins:link PLUGIN`](#oclif-generate-pluginslink-plugin)
* [`oclif-generate plugins:uninstall PLUGIN...`](#oclif-generate-pluginsuninstall-plugin)
* [`oclif-generate plugins:uninstall PLUGIN...`](#oclif-generate-pluginsuninstall-plugin-1)
* [`oclif-generate plugins:uninstall PLUGIN...`](#oclif-generate-pluginsuninstall-plugin-2)
* [`oclif-generate plugins update`](#oclif-generate-plugins-update)

## `oclif-generate hello PERSON`

Say hello

```
USAGE
  $ oclif-generate hello PERSON -f <value>

ARGUMENTS
  PERSON  Person to say hello to

FLAGS
  -f, --from=<value>  (required) Who is saying hello

DESCRIPTION
  Say hello

EXAMPLES
  $ oex hello friend --from oclif
  hello friend from oclif! (./src/commands/hello/index.ts)
```

_See code: [dist/commands/hello/index.ts](https://github.com/axodotdev/oranda/blob/v0.0.0/dist/commands/hello/index.ts)_

## `oclif-generate hello world`

Say hello world

```
USAGE
  $ oclif-generate hello world

DESCRIPTION
  Say hello world

EXAMPLES
  $ oclif-generate hello world
  hello world! (./src/commands/hello/world.ts)
```

## `oclif-generate help [COMMANDS]`

Display help for oclif-generate.

```
USAGE
  $ oclif-generate help [COMMANDS] [-n]

ARGUMENTS
  COMMANDS  Command to show help for.

FLAGS
  -n, --nested-commands  Include all nested commands in the output.

DESCRIPTION
  Display help for oclif-generate.
```

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v5.2.9/src/commands/help.ts)_

## `oclif-generate plugins`

List installed plugins.

```
USAGE
  $ oclif-generate plugins [--core]

FLAGS
  --core  Show core plugins.

DESCRIPTION
  List installed plugins.

EXAMPLES
  $ oclif-generate plugins
```

_See code: [@oclif/plugin-plugins](https://github.com/oclif/plugin-plugins/blob/v2.4.7/src/commands/plugins/index.ts)_

## `oclif-generate plugins:install PLUGIN...`

Installs a plugin into the CLI.

```
USAGE
  $ oclif-generate plugins:install PLUGIN...

ARGUMENTS
  PLUGIN  Plugin to install.

FLAGS
  -f, --force    Run yarn install with force flag.
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Installs a plugin into the CLI.
  Can be installed from npm or a git url.

  Installation of a user-installed plugin will override a core plugin.

  e.g. If you have a core plugin that has a 'hello' command, installing a user-installed plugin with a 'hello' command
  will override the core plugin implementation. This is useful if a user needs to update core plugin functionality in
  the CLI without the need to patch and update the whole CLI.


ALIASES
  $ oclif-generate plugins add

EXAMPLES
  $ oclif-generate plugins:install myplugin 

  $ oclif-generate plugins:install https://github.com/someuser/someplugin

  $ oclif-generate plugins:install someuser/someplugin
```

## `oclif-generate plugins:inspect PLUGIN...`

Displays installation properties of a plugin.

```
USAGE
  $ oclif-generate plugins:inspect PLUGIN...

ARGUMENTS
  PLUGIN  [default: .] Plugin to inspect.

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

GLOBAL FLAGS
  --json  Format output as json.

DESCRIPTION
  Displays installation properties of a plugin.

EXAMPLES
  $ oclif-generate plugins:inspect myplugin
```

## `oclif-generate plugins:install PLUGIN...`

Installs a plugin into the CLI.

```
USAGE
  $ oclif-generate plugins:install PLUGIN...

ARGUMENTS
  PLUGIN  Plugin to install.

FLAGS
  -f, --force    Run yarn install with force flag.
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Installs a plugin into the CLI.
  Can be installed from npm or a git url.

  Installation of a user-installed plugin will override a core plugin.

  e.g. If you have a core plugin that has a 'hello' command, installing a user-installed plugin with a 'hello' command
  will override the core plugin implementation. This is useful if a user needs to update core plugin functionality in
  the CLI without the need to patch and update the whole CLI.


ALIASES
  $ oclif-generate plugins add

EXAMPLES
  $ oclif-generate plugins:install myplugin 

  $ oclif-generate plugins:install https://github.com/someuser/someplugin

  $ oclif-generate plugins:install someuser/someplugin
```

## `oclif-generate plugins:link PLUGIN`

Links a plugin into the CLI for development.

```
USAGE
  $ oclif-generate plugins:link PLUGIN

ARGUMENTS
  PATH  [default: .] path to plugin

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Links a plugin into the CLI for development.
  Installation of a linked plugin will override a user-installed or core plugin.

  e.g. If you have a user-installed or core plugin that has a 'hello' command, installing a linked plugin with a 'hello'
  command will override the user-installed or core plugin implementation. This is useful for development work.


EXAMPLES
  $ oclif-generate plugins:link myplugin
```

## `oclif-generate plugins:uninstall PLUGIN...`

Removes a plugin from the CLI.

```
USAGE
  $ oclif-generate plugins:uninstall PLUGIN...

ARGUMENTS
  PLUGIN  plugin to uninstall

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Removes a plugin from the CLI.

ALIASES
  $ oclif-generate plugins unlink
  $ oclif-generate plugins remove
```

## `oclif-generate plugins:uninstall PLUGIN...`

Removes a plugin from the CLI.

```
USAGE
  $ oclif-generate plugins:uninstall PLUGIN...

ARGUMENTS
  PLUGIN  plugin to uninstall

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Removes a plugin from the CLI.

ALIASES
  $ oclif-generate plugins unlink
  $ oclif-generate plugins remove
```

## `oclif-generate plugins:uninstall PLUGIN...`

Removes a plugin from the CLI.

```
USAGE
  $ oclif-generate plugins:uninstall PLUGIN...

ARGUMENTS
  PLUGIN  plugin to uninstall

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Removes a plugin from the CLI.

ALIASES
  $ oclif-generate plugins unlink
  $ oclif-generate plugins remove
```

## `oclif-generate plugins update`

Update installed plugins.

```
USAGE
  $ oclif-generate plugins update [-h] [-v]

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Update installed plugins.
```
<!-- commandsstop -->
