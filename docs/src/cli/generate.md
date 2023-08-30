# `oranda generate`

> Added in version 0.4.0.

This command generates files useful for working with oranda. Currently, it only supports one subcommand.

## `oranda generate ci`

Generates a CI file that deploys your site. Supports the following options:

- `-o, --output-path`: Specify a path for the file to be written to. Default: `.github/workflows/web.yml`
- `-s, --site-path`: Specify a path to your oranda site, in case it's in a subdirectory to your repository
- `--ci`: Specify which CI platform to use. Currently only accepts and defaults to `github`, which deploys to GitHub 
  Pages using GitHub Actions.

You can rerun this command to update the CI file based on what we currently recommend as the best workflow, but also 
to, for example, update the oranda version that the CI uses (which will always be the oranda version you run 
`generate` with).
