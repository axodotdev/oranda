# Changelogs

oranda supports reading your project's changelogs from GitHub releases and generating pages on your website from your release and release notes. This is automatically enabled if we can find
a GitHub repository for your project, but if you don't want to use this, set the following option:

```json
{
  "components": {
    "changelog": false
  }
}
```

If you have a `CHANGELOG(.md)` file, oranda will attempt to parse your changelog
contents for the respective versions, and embed them into the generates page(s). This file needs to be valid Markdown,
and contain a valid header structure like this:

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

## Changelog settings

The changelog option supports the following sub-settings:

- [`read_changelog_file`](#read_changelog_file) - disable reading the changelog file

### read_changelog_file

> Added in version 0.3.0.

Disables reading the changelog file, meaning that oranda will fall back to embedding the GitHub release body instead.
Defaults to `true`.


