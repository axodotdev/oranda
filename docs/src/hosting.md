# Hosting

## On GitHub pages

When hosting on Github pages, it is often the case that your site will be served from a non-root url such as `myorg.github.io/reponame`.

If you are serving your site from this style of URL, you'll need to add the `reponame` as a path_prefix to your Oranda config. This will allow oranda to properly configure all chlid links (such as images or additional pages) of your page to be properly name spaced.

Let's say your website is hosted at `https://myorg.github.io/reponame/`, to fix the links, add the following to your `oranda.json`

```json
{
  "build": {
    "path_prefix": "reponame"
  }
}
```

### Using GitHub Actions

You can set up a simple workflow to automatically do this GitHub Pages deploy for you. Take a look at Oranda's own
[web.yml] file for reference:

```yaml
# Workflow to build your docs with oranda (and mdbook)
# and deploy them to Github Pages
name: Web deploy

# We're going to push to the gh-pages branch, so we need that permission
permissions:
  contents: write

# What situations do we want to build docs in?
# All of these work independently and can be removed / commented out
# if you don't want oranda/mdbook running in that situation
on:
  # Check that a PR didn't break docs!
  #
  # Note that the "Deploy to Github Pages" step won't run in this mode,
  # so this won't have any side-effects. But it will tell you if a PR
  # completely broke oranda/mdbook. Sadly we don't provide previews (yet)!
  pull_request:

  # Whenever something gets pushed to main, update the docs!
  # This is great for getting docs changes live without cutting a full release.
  #
  # Note that if you're using cargo-dist, this will "race" the Release workflow
  # that actually builds the Github Release that oranda tries to read (and
  # this will almost certainly complete first). As a result you will publish
  # docs for the latest commit but the oranda landing page won't know about
  # the latest release. The workflow_run trigger below will properly wait for
  # cargo-dist, and so this half-published state will only last for ~10 minutes.
  #
  # If you only want docs to update with releases, disable this one.
  push:
    branches:
      - main
  
  # Whenever a workflow called "Release" completes, update the docs!
  #
  # If you're using cargo-dist, this is recommended, as it will ensure that
  # oranda always sees the latest release right when it's available. Note
  # however that Github's UI is wonky when you use workflow_run, and won't
  # show this workflow as part of any commit. You have to go to the "actions"
  # tab for your repo to see this one running (the gh-pages deploy will also
  # only show up there).
  workflow_run:
    workflows: ["Release"]
    types:
      - completed

# Alright, let's do it!
jobs:
  web:
    name: Build and deploy site and docs
    runs-on: ubuntu-latest
    steps:
      # Setup
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0
      - uses: dtolnay/rust-toolchain@stable
      - uses: swatinem/rust-cache@v2

      # Install and run oranda (and mdbook)
      # This will write all output to ./public/ (including copying mdbook's output to there)
      - name: Install and run oranda
        run: |
          curl --proto '=https' --tlsv1.2 -LsSf https://github.com/axodotdev/oranda/releases/latest/download/oranda-installer.sh | sh
          oranda build

      # Deploy to our gh-pages branch (making it if it doesn't exist)
      # the "public" dir that oranda made above will become the root dir
      # of this branch.
      #
      # Note that once the gh-pages branch exists, you must
      # go into repo's settings > pages and set "deploy from branch: gh-pages"
      # the other defaults work fine.
      - name: Deploy to Github Pages
        uses: JamesIves/github-pages-deploy-action@v4.4.1
        # ONLY if we're on main (so no PRs or feature branches allowed!)
        if: ${{ github.ref == 'refs/heads/main' }}
        with:
          branch: gh-pages
          # Gotta tell the action where to find oranda's output
          folder: public
          token: ${{ secrets.GITHUB_TOKEN }}
          single-commit: true
```

## Elsewhere

oranda is, effectively, a static site generator. It outputs HTML, CSS and JavaScript files. These can all be hosted on a
looooot of different platforms, in fact, too many for us to enumerate here! You can use Vercel, Netlify, any GitHub pages
competitor, or you can plop it on your own server that runs nginx, Apache httpd, Caddy, or anything else!

You can, in fact, also use the CI example linked above and modify it to deploy to different platforms. If you do,
we'd love to hear about it!

[web.yml]: https://github.com/axodotdev/oranda/blob/main/.github/workflows/web.yml