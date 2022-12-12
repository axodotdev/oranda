# oranda
> beautiful custom websites for your CLI

## Usage

- `build`: generates a static site based on project and/or custom configuration.
  optionally takes a `--path` flag, otherwise looks in the current directory
- `serve`: starts a local server to preview site generated from `build` command.
  uses `dist_dir` to find assets, which defaults to `public`. optionally takes
  a `--port` flag, otherwise uses `7979`.
