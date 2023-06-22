# Adding static assets

If you reference static assets in your Markdown, you'll need to place them all inside a directory at the same level as
your project manifest file called `static`. This is because Oranda currently doesn't know about each indidivual asset,
and instead just copies the folder where they're contained.

In your Markdown, you'll need to refer to the assets in this directory. For example:

```md
![An image from my amazing project](./static/project.png)
```

If you want to use a custom-named directory you can configure this in your `oranda.json`, like so:

```json
{
  "build": {
    "static_dir": "assets"
  }
}
```

In this case the `assets` directory will be used instead of the default `static` directory.
