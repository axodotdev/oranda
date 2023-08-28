# Changelogs

oranda can generate a separate changelog file from either a local `CHANGELOG.md` file in your repository, or from the body
of GitHub releases. This setting is **enabled** by default, as long as you have a repository set for your project. To disable this
feature, set it to false in the `oranda.json`:

```json
{
  "components": {
    "changelog": false
  }
}
```

## Controlling where changelogs are read from

By default, oranda will try to read changelog contents from a file called `CHANGELOG(.md)`. This file needs to be formatted
in such a way that it can be parsed, meaning you'll have to specify consistent headers in your Markdown file, like this:

```markdown
# Changelog

## 0.1.1 - 2023-04-05

- Fixed things

## 0.1.0 - 2023-04-02

### New features

- Fancy thingie
- Other cool stuff

### Fixes

- Beep booping is now consistent
```

If you would like oranda to use the bodies of GitHub releases that it finds instead, set the following option:

```json
{
  "components": {
    "changelog": {
      "read_changelog_file": false
    }
  }
}
```

> Even if oranda reads from a local changelog file, it will still try to match those releases to GitHub releases. Make
> sure that both version numbering schemes are the same between your local changelog and GitHub releases.

For a complete reference of changelog configuration, consult the [reference](./reference.md#componentschangelog)

## For workspaces

If you have a [workspace](./workspaces.md), but you would like to opt-out of changelogs for only some members, you'll need
to add manual overrides in those `oranda.json` files right now.