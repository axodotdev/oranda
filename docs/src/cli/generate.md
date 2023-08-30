# `oranda generate`

> Added in version 0.4.0.

This command generates files useful for working with oranda. Currently, it only supports one subcommand.

## `oranda generate ci`

Generates a CI file that deploys your site to GitHub Pages. Supports the following options:

- `-p, --path`: Specify a path for the file to be written to. Default: `.github/workflows/web.yml`

You can rerun this command to update the CI file based on what we currently recommend as the best workflow, but also 
to, for example, update the oranda version that the CI uses (which will always be the oranda version you run 
`generate` with).
