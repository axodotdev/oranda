# Configuration Reference

- [`project`](#project)
    - [`name`](#projectname) 📦 - the name of your application
    - [`version`](#projectversion) 📦 - current version of your project
    - [`description`](#projectdescription) 📦 - brief description of your project
    - [`homepage`](#projecthomepage) 📦 - url to the homepage of your project
    - [`repository`](#projectrepository) 📦 - url to the repository of your project
    - [`readme_path`](#projectreadme_path) - relative custom path to your project's readme file
    - [`license`](#projectlicense) 📦 - license of your project (in SPDX format)
- [`build`](#build)
    - [`dist_dir`](#builddist_dir) - path to where built output should be placed
    - [`static_dir`](#buildstatic_dir) - path to a directory containing static assets
    - [`path_prefix`](#buildpath_prefix) - a URL fragment to prepend to URLs, useful if hosting from a subfolder
    - [`additional_pages`](#buildadditional_pages) - additional pages to be rendered and linked to
- [`marketing`](#marketing)
    - [`analytics`](#marketinganalytics) - automatically insert analytics snippets for several providers
    - [`social`](#marketingsocial) - additional configuration for SEO-related inserts
- [`styles`](#styles)
    - [`theme`](#stylestheme) - change oranda's CSS theme
    - [`additional_css`](#stylesadditional_css) - additional CSS to insert into your pages
    - [`oranda_css_version`](#stylesoranda_css_version) - custom version of oranda's built-in CSS to use
    - [`logo`](#styleslogo) - custom site logo
    - [`favicon`](#stylesfavicon) - custom site favicon
- [`components`](#components)
    - [`changelog`](#componentschangelog) - extract your changelog from GitHub automatically
    - [`mdbook`](#componentsmdbook-or-componentsmd_book) - let us render a mdbook site for you
    - [`funding`](#componentsfunding) - configuration for rendering a site related to project funding methods
    - [`artifacts`](#componentsartifacts) - configuration for displaying downloadable artifacts/installers
- [`workspace`](#workspace) - **`oranda-workspace.json` only!!**
  - [`name`](#workspacename) - set the overarching workspace name
  - [`auto`](#workspaceauto) - enable workspace autodetection
  - [`generate_index`](#workspacegenerate_index) - disable generating a workspace index page
  - [`members`](#workspacemembers) - list the workspace members
  - [`docs_path`](#workspacedocs_path) - path to a markdown file to embed into your workspace index page
  - [`preferred_members`](#workspacepreferred_members) - list of workspace members to highlight at the top of the index page

> 📦 = automatically collected from your package metadata!

## project

Configuration for metadata about your project. Most of the info in here can be automatically collected
for Cargo and npm projects.

### project.name

> Added in version 0.1.0.

- Type: string, Default: Project manifest `name` field

Your project's name. Will be used for the page title and header.

### project.version

> Added in version 0.1.0.

- Type: string, Default: Project manifest `version` field.

Your project's current version.

### project.description

> Added in version 0.1.0.

- Type: string, Default: Project manifest `description` field

Your project's description. Will be used for site metadata.

### project.homepage

> Added in version 0.1.0.

- Type: string, Default: Project manifest `homepage` field

Your project's homepage. Will be used for backlinks and site metadata.

### project.repository

> Added in version 0.1.0.

- Type: string, Default: Project manifest `repository` field

Your project's Git repository. If set to GitHub, this enables `oranda` to fetch data from GitHub releases
(see [artifacts](./artifacts.md))

### project.readme_path

> Added in version 0.1.0.

- Type: string, Default: A variation of the standard `README.md`

The path to your project's readme file. The contents of this will be used for your index page.

### project.license

> Added in version 0.1.0.

- Type: string, Default: Project manifest `license` field.

Your project's license. Will be embedded into the page footer.

## build

Configuration regarding the specifics of how and where you want your site built.

### build.dist_dir

> Added in version 0.1.0.

- Type: string, Default: `public/`

The directory where your static files will be output to. This must be relative to the `oranda.json` file.

### build.static_dir

> Added in version 0.1.0.

- Type: string, Default: `static/`

Static content that oranda will copy to its output folder. This must be relative to the `oranda.json` file.

### build.path_prefix

> Added in version 0.1.0.

- Type: string, Default: none

If you're hosting oranda on a nested path (e.g. `mysite.cool/myproject`), you should set `path_prefix` to
`myproject` in your configuration in order for oranda to generate correct links. This is specifically useful for
GitHub pages, which, unless the repository name is `username.github.io` or you have a custom domain set, will host
projects in a subfolder (e.g. `username.github.io/projectname`, so you'd set this option to `projectname`).

### build.additional_pages

> Added in version 0.1.0.

- Type: object, Default: none

An object of additional Markdown pages that you'd like to be included. Links to these will appear in the site header,
and they will all be rendered into separate pages.

[More information](./additional-pages.md)

## marketing

Configuration regarding SEO, site metadata, and other "marketing"-related aspects of your page.

### marketing.analytics

> Added in version 0.1.0.

- Type: object, Default: none

[More information](./analytics.md)

Configuration for page analytics. Can be any combination of the following:

#### marketing.analytics.google_analytics

> Added in version 0.1.0.

- Type: object, Default: none

Set `google_analytics.tracking_id` to your site tracking ID to include the relevant
snippet to your page.

#### marketing.analytics.plausible

- Type: object, Default: none

> Added in version 0.1.0.

Set `plausible.domain` to your Plausible domain. Optionally, you can set `plausible.script_url`
if you're self-hosting.


#### marketing.analytics.fathom

- Type: object, Default: none

> Added in version 0.1.0.

Set `fathom.site` to your Fathom site.

#### marketing.analytics.unami

- Type: object, Default: none

Set `unami.website` to your Unami website identifier, and `unami.script_url` to the location
where you're hosting your Unami script.

### marketing.social

> Added in version 0.1.0.

- Type: object, Default: none

[More information](./social.md)

Options useful for SEO features.

#### marketing.social.image

> Added in version 0.1.0.

- Type: string, Default: none

An image URL to use for page embeds.

#### marketing.social.image_alt

> Added in version 0.1.0.

- Type: string, Default: none

An alt text for said image embed.

#### marketing.social.twitter_account

> Added in version 0.1.0.

- Type: string, Default: none

Name of a Twitter/X account, to be used for Twitter/X embeds (including the `@`).

## styles

- Type: object

Configuration regarding the looks of your site.

### styles.theme

> Added in version 0.1.0.

- Type: string, Default: `dark`

[More information](./theme.md)


Choose which built-in theme to use. Possible choices:

- `dark` (default)
- `light`
- `axodark`
- `axolight`
- `hacker`
- `cupcake`

### styles.additional_css

> Added in version 0.1.0.

- Type: array, Default: none

[More information](./theme.md#customizing-themes)

An array of local or remote CSS files that will be merged together and loaded into your page.

### styles.oranda_css_version

> Added in version 0.1.0.

- Type: string, Default: none (current version)

Specify a version of the embedded oranda CSS. This can be used to opt into newer CSS releases that don't have
an oranda release associated with them yet. (Internally, this looks for a `oranda.css` release artifact on the given
tag in the `axodotdev/oranda` GitHub repository)

### styles.logo

> Added in version 0.1.0.

- Type: string, Default: none

Path to a custom logo to be shown in your website header and in your site metadata.

### styles.favicon

> Added in version 0.1.0.

- Type: string, Default: none

Path to a custom favicon.

## components

Configuration regarding extra components/functionality that oranda supports.

### components.artifacts

> Added in version 0.1.0.

- Type: object or bool

[More information](./artifacts.md)

Configuration for enabling downloadable artifacts, as well as the `cargo-dist` integration.

#### components.artifacts.package_managers

> Added in version 0.1.0.

- Type: object, Default: none

A list of "package manager"-like ways to install your app. These will be displayed on your
page as extra runnable commands that users can execute to download your project. There's a few
different "states" these can be in:

- `package_managers.preferred` - Entries here will be shown in the install widget on your front page
- `package_managers.additional` - Entries here will only be shown on the "install" page, but not on the front page

The syntax for both of these is the same:

```json
{
  "components": {
    "artifacts": {
      "package_managers": {
        "preferred": {
          "user-friendly name": "command to run",
          "for example, npm": "npm install @axodotdev/oranda --save-dev"
        }
      }
    }
  }
}
```

#### components.artifacts.cargo_dist

> Added in version 0.1.0.

- Type: bool, Default: `true` if we detected support, `false` otherwise

Enables/disables `cargo-dist` support. oranda may autodetect this if you have `cargo-dist`
configuration in your `Cargo.toml`, but you can always explicitly disable it here.

#### components.artifacts.auto

> Added in version 0.2.0.

- Type: bool, Default: `false`

Enables/disables artifacts autodetection, even without `cargo-dist`. This is turned off by
default, but if you provide GitHub release artifacts in a target-triple-like format, chances
are that oranda can autodetect them, so it may be worth turning this on.

### components.artifacts.match_package_names

> Added in version 0.5.0.

- Type: bool, Default: `false`

Only uses release tags that contain the name of the project being generated. Useful in a workspace environment,
where multiple published projects are stored in the same repository.

### components.mdbook (or components.md_book)

> Added in version 0.1.0.

- Type: object or bool

[More information](./mdbook.md)

Configuration for mdbook support. You don't need mdbook itself installed to make use of this,
since it also provides a Rust library that we use. oranda will attempt to autodetect
this at several common paths, so you can disable it by setting `components.mdbook` to `false`.

#### components.mdbook.path

> Added in version 0.1.0.

- Type: string, Default: none

Path to your mdbook directory, the one containing `book.toml`, relative to your
configuration file.

#### components.mdbook.theme

> Added in version 0.1.0.

- Type: bool, Default: `true`

Whether to enable or disable custom mdbook themes. We try to match your mdbook to
the main oranda page look visually by default, but you can disable this by setting this
option to `false`.

### components.changelog

> Added in version 0.1.0.

- Type: object or bool

[More information](./changelog.md)

Enable/disable changelog generation. This is enabled if you have a repository URL set,
and you can disable it by setting `false` here.

#### components.changelog.read_changelog_file

> Added in version 0.3.0.

- Type: bool, Default: `true`

Disables reading the changelog file, meaning that oranda will fall back to embedding the GitHub release body instead.

#### components.changelog.generate_rss_feed

> Added in version 0.5.0.

- Type: bool, Default: `true`

Disables the built-in generation of a RSS feed file for your changelog.

### components.funding

> Added in version 0.1.0.

- Type: object or bool

[More information](./funding.md)

Allows you to tweak or disable oranda's funding page.

#### components.funding.md_path

> Added in version 0.1.0.

- Type: string, Default: none

Path to a Markdown file which will be embedded into the funding page.

#### components.funding.yml_path

> Added in version 0.1.0.

- Type: string, Default: `.github/FUNDING.yml`

Custom path to the GitHub-formatted `FUNDING.yml` file.

## workspace

[More information](./workspaces.md)

Configuration for a workspace. This option and its sub-keys will only be honored
in the `oranda-workspace.json` file, in a normal configuration file, they will be
ignored.

### workspace.name

> Added in version 0.3.0.

- Type: string, Default: My Oranda Workspace

Set the overarching workspace name. This is optional, and will fall back to "My Oranda Workspace" if not set (not very
intuitive, I know).

### workspace.auto

> Added in version 0.3.0.

- Type: bool, Default: `false`

Enables workspace autodetection if set to `true`. This will cause oranda to attempt to find any Cargo or NPM workspaces
under the current directory, and to attempt to build all of its members (all members must therefore have at least a
readme file). Members manually listed under the `members` key override these automatically detected workspace members.

### workspace.generate_index

> Added in version 0.3.0.

- Type: bool, Default: `true`

If set to `false`, does not generate a workspace index page that links between all workspace members. Use this if you
just want to use oranda's workspace functionality to build multiple unrelated sites in one go.

### workspace.members

> Added in version 0.3.0.

- Type: array, Default: none

An array of objects representing the workspace members.

#### workspace.members.slug

> Added in version 0.3.0.

- Type: string

The URL-safe slug this page will be built at. This needs to be something that can be parsed as a URL, as well as a folder
name on your target system (because oranda is a static site generator, after all).

#### workspace.members.path

> Added in version 0.3.0.

- Type: string

The path to the page source. Point this to the same directory that the `oranda.json` would be in.

### workspace.docs_path

> Added in version 0.3.0.

- Type: string, Default: none

Path to a Markdown file whose content is going to be rendered into the workspace index file.

### workspace.preferred_members

> Added in version 0.3.0.

- Type: array, Default: none

A list of workspace member `slug`s that should be highlighted at the top of the index page. For example:

```json
{
  "workspace": {
    "auto": true,
    "preferred_members": ["projectone", "projecthree"]
  }
}
```
