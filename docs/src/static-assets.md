# Adding static assets

If you reference static assets in your markdown, you'll need to place them all inside a directory at the root of your project called `static`.

In your markdown, you'll need to refer to the assets in this directory. For example:

```md
![An image from my amazing project](./static/project.png)
```

If you wish to use a custom-named directory you can configure this in your `oranda.json`, like so:

```json
{
  "static_dir": "assets"
}
```

In this case the `assets` directory will be used instead of the default `static` directory.
