# Funding page

Oranda has the capability of reading information from your GitHub funding file, and
automatically writing a page based on it. Unless you disable it by passing
`"funding": false` in the oranda config file, oranda will search your project for
a `.github/FUNDING.yml` file, and generate a page based off of it. You can read
more about the format of this file on [GitHub's docs][funding-docs].

Oranda will display your different sponsor/funding links next to each other, but
if have a "main" funding option, you can set the following configuration setting:

```json
{
  "components": {
    "funding": {
      "preferred_funding": "github"
    }
  }
}
```

Make sure this key corresponds to one of the possible entries in the `FUNDING.yml`
file.

If you want to display additional information or context, oranda can also include
the contents of a top-level `funding.md` Markdown file. Its contents will be translated
into HTML and displayed on the Funding page as well.

Both of the YAML and Markdown file paths can be customized as such:

```json
{
  "components": {
    "funding": {
      "md_path": "myfunding.md",
      "yml_path": "misc/funding.yml"
    }
  }
}
```

> oranda's funding parsing and site generation are currently an experiment into how
  to better integrate common funding methods into your tools' websites. If you have
  any feedback on how we could do things better, let us know on
  [Discord][axodiscord] or [GitHub][newissue]!

[funding-docs]: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/displaying-a-sponsor-button-in-your-repository
[axodiscord]: https://discord.com/invite/wVqCRGsb
[newissue]: https://github.com/axodotdev/oranda/issues/new