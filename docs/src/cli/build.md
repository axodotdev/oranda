# `oranda build`

This command builds your oranda site. You can specify:

- **The project root** (`--project-root`), in case you want to build from another directory
- **The config path** (`--config-path`), if your configuration file is not `./oranda.json`

You can also pass the `--json-only` flag in order for oranda to _only_ build an `artifacts.json` file that can
be read by other tools (or websites) for integration purposes.
