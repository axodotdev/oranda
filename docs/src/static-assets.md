# Adding static assets

If in your website you would like to point to any sort of static assets may those be images, binaries or anything in between you can put these assets in the `static` directory and this whole directory will then be copied over to the built website.

A example of an image link on markdown is:

```md
![An image from my amazing project](./static/project.png)
```

If you wish to use another folder instead you can customize the folder used in the `oranda.json`, like so:

```json
{
  "static_dir": "assets"
}
```

In this case the `assets` folder will be used.
