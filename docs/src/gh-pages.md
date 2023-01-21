# Hosting on Github Pages

When hosting on Github pages, it is often the case that your site will be served from a non-root url such as `myorg.github.io/reponame`.

If you are serving your site from this style of URL, you'll need to add the `reponame` as a path_prefix to your Oranda config. This will allow oranda to properly configure all chlid links (such as images or additional pages) of your page to be properly name spaced."

Let's say your website is hosted at `https://myorg.github.io/reponame/`, to fix the links add the following to your `oranda.json`

```json
{
  "path_prefix": "oranda"
}
```
