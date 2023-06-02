# `oranda dev`

This command basically combined `oranda build` and `oranda serve`, with the added benefit of watching for changes
and recompiling automatically. When you launch, what happens is this:

1. Oranda builds your site (unless you told it not to)
2. Oranda launches a server similar to `oranda serve`
3. Oranda starts watching its relevant files for changes, and will rerun the build process when something changes

Oranda's build can have a lot of side-effects (reading/writing files, but also talking to the GitHub API), and as
such, we have to take care to only run the build process when _relevant_ files change. These files are:

- Your project manifest files (`Cargo.toml`, `package.json`)
- Your oranda configuration file
- Any mdbook source files you may have
- Your readme, and additional files specified in the configuration
- Any other paths you give it using `--include-paths`

This command also support several options:

- `--port` to set a custom port for the file server
- `--project-root` to change the root directory from where your site will be built
- `--config-path` to specify a custom path for your oranda config
- `--no-first-build` to skip the first step mentioned above where oranda builds your site before starting the watch process
- `-i`, `--include-paths` to specify custom paths for oranda to watch
