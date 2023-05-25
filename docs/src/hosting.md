# Hosting

## On GitHub pages

When hosting on Github pages, it is often the case that your site will be served from a non-root url such as `myorg.github.io/reponame`.

If you are serving your site from this style of URL, you'll need to add the `reponame` as a path_prefix to your Oranda config. This will allow oranda to properly configure all chlid links (such as images or additional pages) of your page to be properly name spaced."

Let's say your website is hosted at `https://myorg.github.io/reponame/`, to fix the links add the following to your `oranda.json`

```json
{
  "path_prefix": "reponame"
}
```

## Elsewhere

oranda is, effectively, a static site generator. It outputs HTML, CSS and JavaScript files. These can all be hosted on a
looooot of different platforms, in fact, too many for us to enumerate here! You can use Vercel, Netlify, any GitHub pages
competitor, or you can plop it on your own server that runs nginx, Apache httpd, Caddy, or anything else!
