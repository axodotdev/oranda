# Changelogs

oranda supports reading your project's changelogs from GitHub releases. You can enable this by setting `changelog` to `true`:

```json
{
  "components": {
    "changelog": true
  }
}
```

This will result in a new "Changelog" page being generated. Changelogs are pulled directly from GitHub releases. If
you're using the [`cargo-dist` integration](./artifacts.md), oranda will attempt to parse a `CHANGELOG.md`-like file for
the changelogs instead.

> **NOTE:** We're working on getting changelog parsing from a `CHANGELOG.md` file as a default feature, without requiring
  use of `cargo-dist`!
