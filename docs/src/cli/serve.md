# `oranda serve`

This command launches a small [`axum`][axum]-powered server that serves your generated oranda site.

Importantly, this does **not** build your site for you. If it can't find a build in the `public/` directory,
it will error and exit. You can set the port for the server to be launched using the `--port` option.

[axum]: https://cra.tw/axum
