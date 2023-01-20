# Hosting on Github Pages

So you have set up oranda, set up some links and deployed it to Github pages with CI and now the links won't work?

The issue here is that github pages does not host your site at the root of the domain and we assume that all oranda deploys will be at the root, to fix this we have a option you can pass in your `oranda.json` called `path_prefix` that let's you set up that.

Let's say your website is hosted at `https://acme.github.io/oranda/`, to fix the links add the following to your `oranda.json`

```json
{
  "path_prefix": "oranda"
}
```
