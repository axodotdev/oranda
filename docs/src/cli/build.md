# `oranda build`

This command builds your oranda site. You can pass the `--json-only` flag in order for oranda to _only_ build an
`artifacts.json` file that can be read by other tools (or websites) for integration purposes. You can also specify
`--config-path` if your configuration file is not `./oranda.json`, but oranda will still look for an
`oranda-workspace.json` in the current directory.
