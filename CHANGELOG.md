# Changelog

## 0.1.0

## What is oranda?

oranda is a hands-off static site generator for people who want a website for their tool but don't want to learn a static site generator. As long as you have a `README.md` in your directory, you can benefit from oranda. oranda will also try to automatically work with:

- Release artifacts (currently only for GitHub releases)
    - `cargo-dist`-generated ones, and
    - arbitrary release artifacts
- `mdbook`docs
- GitHub-supported project/maintainer funding sources

oranda is designed to _just work_ in a lot of cases, and in cases where it doesn't, to provide fine-grained configuration so you can make it work for you.

## Features

### Components

#### Github release artifacts inference

We can now not only figure out whether you're using `cargo-dist` _automatically_, but we also try and support arbitrary tarballs, as long as they're attached to a release and they're following the target-triple format. Oranda will now, for example, pick up a release artifacts called `myapp-aarch64-apple-darwin.tar.xz`, even if the project isn't using `cargo-dist`.

#### Smarter install widget

The installer widget on our main page has been upgraded! It now not only shows a select box where you can switch between different architectures (though we still attempt to figure out what platform you're running on), but it now additionally displays your package managers:

![](https://hackmd.io/_uploads/B1iKMxju3.png)

You can customize which package managers you want to be displayed on in this widget vs. which ones should only be displayed on the separate install page, as well.

#### Funding page

oranda now has the ability to autodetect whether you're using GitHub's funding embed functionality (meaning you have a `.github/FUNDING.yml`), in which case it'll automatically generate a page showing your available funding options.

Additionally, you can enhance this page by selecting a particular funding channel to be showcased as well as providing custom content from a `funding.md` Markdown file. You can, of course, fully disable this feature, or adjust the paths if you want.

This is less of a defined feature and more of an experiment on how we can better integrate maintainers' funding sources onto their websites. Please let us know what you think, or if you have any other feedback or input!

#### `mdbook` autodetect and styling

oranda themes now get applied to your mdbook output, too, meaning there's less of a discrepancy between your flashy oranda page and your default-mdbook-styled docs. We've also been hard at work being able to detect when you use `mdbook` without you having to set it, which should now work in the majority of cases.

### Configuration

#### New configuration structure

We've completely revamped our configuration structure to be more future-proof and to require less refactoring in the fullness of time. For the new configuration layout, please [consult the docs](https://opensource.axo.dev/oranda/book/configuration.html).

One other major change is that we now **reject unknown config keys**. This means that if you've had a oranda 0.0.3 site, it will now force you to migrate your config to the new format. We've decided on this because we believe that doing anything but hard erroring in this situation would lead to unwanted behavior (old keys getting interpreted in new, weird ways, and so on).

#### Config schema

oranda's configuration schema is now available in a JSON schema for each release starting with this one. This means that in editors like VSCode, you can attach the schema to your `oranda.json` file and get autofill and documentation, like this:

```json
{
  "$schema": "https://github.com/axodotdev/oranda/releases/download/v0.1.0/oranda-config-schema.json"
}
```

### CLI

#### `dev` command

This release introduces `oranda dev`, which bundles both building your site and serving it from a file server, as well as sprinkling in auto-recompilation when files change. It's intended to be the prime command for local development of a oranda site.

## Bug fixes

(this is a selection, there's been way too many to fully list, at least until we're able to automatically generate a list)

- Various style fixes, lists now display correct, colors should be less offensive to the eye, that sort of stuff
- We're much better now at handling complex release histories!
- We've completely removed the `version` key from the configuration. It wasn't used, and we probably won't use it in the future, either

## Docs

Documentation has had a major rewrite! We now provide a full configuration overview, as well as more detailed writeups for major parts of functionality.

## Thank you to our contributors!

Despite being formally unannounced, several intrepid folks discovered oranda, and have been using it for personal projects and contributing issues and PRs. Their feedback has been invaluable in getting oranda to 0.1.0 today and we'd like to thank them:

- [2mill]: https://github.com/2mill
- [andrewmd5]: https://github.com/andrewmd5
- [jamesmunns]: https://github.com/jamesmunns
- [MarcoIeni]: https://github.com/MarcoIeni 
- [msjarvis]: https://github.com/msfjarvis
- [pomdtr]: https://github.com/pomdtr
- [proofconstruction]: https://github.com/proofconstruction
- [tshepang]: https://github.com/tshepang
- [untitaker]: https://github.com/untitaker
- [zkat]: https://github.com/zkat

## 0.0.3 - 2023-05-08

### Features

- **Individual Changelog pages: [shadows-withal]/[↬284]**
    
    When announcing a new release- it's often desirable to link to an individual
    release page that contains the changelog/release notes. Previously, we built
    a single page for all the releases- now we build individual pages as well.

    This is the first shipped feature from our new team member, Liv! Yay and 
    welcome :)

- **npm installer: [ashleygwilliams]/[↬288]**

    As of 0.0.6, `cargo-dist` will build an npm installer for you! So now you
    can npm or npx oranda!

### Fixes

- **Improved configuration support for non-cargo dist users: [pomdtr]/[#262] , [ashleygwilliams]/[↬281]**

    Previously, setting `cargo-dist` as false, or omitting it should have been
    sufficient to stop oranda from attempting to parse your releases as
    cargo-dist artifacts, however `cargo-dist: false` did not work! How the
    entire artifacts config object is handled has been reworked and tested.
    Additionally, work from a refactor has also made projects with a mixture of
    cargo-dist and non-cargo-dist releases work much better.

- **Installer header fallback when platform is not detected: [gankra]/[↬269]**

    When you use cargo-dist, we display a header on your index page that
    detects your user's platform and recommends and installer to them. We did
    not previously have a fallback if we detected a system that none of the
    installers supported. Now, in that scenario- we'll offer an artifact
    download as the header option.

- **Dev commands has proper default values: [ashleygwilliams]/[#256],[↬260]**

    Due to a false hope that `#[derive(Default)]` would collect defaults from 
    the `clap` derive API, we shipped the `dev` command with each argument's
    *type's* defaults, not the oranda ones. `dev` now has the same defaults
    that `serve` does, as is to be expected.

- **Improved typography and layout styles on mobile: [ashleygwilliams]/[#234], [SaraVieira]/[↬276]**

### Maintenance

- **Re-add mdbook: [ashleygwilliams]/[#190], [gankra]/[↬285]**
- **Data fetching refactor: [ashleygwilliams]/[#226],[↬274]**
- **Update actions/checkout to v3: [ashleygwilliams]/[#251],[↬255]**

[#190]: https://github.com/axodotdev/oranda/issues/190
[#209]: https://github.com/axodotdev/oranda/issues/209
[#226]: https://github.com/axodotdev/oranda/issues/226
[#234]: https://github.com/axodotdev/oranda/issues/234
[↬240]: https://github.com/axodotdev/oranda/pull/240
[#251]: https://github.com/axodotdev/oranda/issues/251
[↬255]: https://github.com/axodotdev/oranda/pull/255
[#256]: https://github.com/axodotdev/oranda/issues/256
[↬260]: https://github.com/axodotdev/oranda/pull/260
[#262]: https://github.com/axodotdev/oranda/issues/262
[↬269]: https://github.com/axodotdev/oranda/pull/269
[↬274]: https://github.com/axodotdev/oranda/pull/274
[↬276]: https://github.com/axodotdev/oranda/pull/276
[↬281]: https://github.com/axodotdev/oranda/pull/281
[↬284]: https://github.com/axodotdev/oranda/pull/284
[↬285]: https://github.com/axodotdev/oranda/pull/285
[↬288]: https://github.com/axodotdev/oranda/pull/288

[ashleygwilliams]: https://github.com/ashleygwilliams
[pomdtr]: https://github.com/pomdtr
[SaraVieira]: https://github.com/SaraVieira
[gankra]: https://github.com/gankra
[shadows-withal]: https://github.com/shadows-withal

## 0.0.2 - 2023-04-13

### Features

- **Changelogs - [ashleygwilliams]/[#192], [SaraVieira]/[↬193]**

- **`oranda dev` command v1: [ashleygwilliams]/[#209],[↬240]**

    It is common to run `oranda build && oranda serve` so `oranda dev` now wraps
    that into a single function. A future version will improve this to include
    watching to automatically rebuild on changes, while serve continues to run
    uninterrupted.`

- **Additional pages takes a hashmap to allow custom nav names: [ashleygwilliams]/[#115], [SaraVieira]/[↬203]**

    Previously, the nav element for an additional page was inferred from its filename.
    However, this led to some unfortunate presentations- especially if the additional
    page had an all caps filename while others did not. This new data structure
    allows you to provide the specific string you'd like to see in the nav for
    any additional page.

- **Improved error handling: [spitfire05]/[#206], [ashleygwilliams]/[↬191], [SaraVieira]/[↬193],[↬201]**

    Several PRs over the course of the last milestone have dramatically improved
    error handling: from replacing panics with proper errors to adding more
    descriptive errors with messages that help you better understand and solve
    the issue.

- **Themes: [SaraVieira]/[↬208]**

    There are now 2 additional themes- light(default), dark, hacker, and cupcake.

- **Version number and publish date on installer header: [ashleygwilliams]/[#194], [SaraVieira]/[↬217]**

- **Improved styling: [SaraVieira]/[↬202],[↬220]**

### Fixes

- **Code block with unsupported highlight annotations should show as plaintext: [spitfire05]/[#236], [SaraVieira]/[↬237]**

    Annotating a code block with an unsupported language highlight led to blocks
    simply not rendering at all. Now they will render as plainttext and will show
    a warning to request support.

- **Nav should show if using dist/changelog, but not additional pages: [ashleygwilliams]/[#183],[↬187]**

    Originally, we would only generate a nav if a user configured additional
    markdown pages. However, nav is also needed if a user configures their
    project to use cargo-dist or render changelogs.

### Maintenance

- **Update cargo-dist to enable Mac ARM builds: [pomdtr]/[#198], [ashleygwilliams]/[↬248]**

[ashleygwilliams]: https://github.com/ashleygwilliams
[pomdtr]: https://github.com/pomdtr
[SaraVieira]: https://github.com/SaraVieira
[spitfire05]: https://github.com/spitfire05

[#115]: https://github.com/axodotdev/oranda/issues/115
[#183]: https://github.com/axodotdev/oranda/issues/183
[#192]: https://github.com/axodotdev/oranda/issues/192
[#194]: https://github.com/axodotdev/oranda/issues/194
[#198]: https://github.com/axodotdev/oranda/issues/198
[#206]: https://github.com/axodotdev/oranda/issues/206
[#209]: https://github.com/axodotdev/oranda/issues/209
[#236]: https://github.com/axodotdev/oranda/issues/236

[↬187]: https://github.com/axodotdev/oranda/pull/187
[↬191]: https://github.com/axodotdev/oranda/pull/191
[↬193]: https://github.com/axodotdev/oranda/pull/193
[↬201]: https://github.com/axodotdev/oranda/pull/201
[↬202]: https://github.com/axodotdev/oranda/pull/202
[↬203]: https://github.com/axodotdev/oranda/pull/203
[↬208]: https://github.com/axodotdev/oranda/pull/208
[↬217]: https://github.com/axodotdev/oranda/pull/217
[↬220]: https://github.com/axodotdev/oranda/pull/220
[↬237]: https://github.com/axodotdev/oranda/pull/237
[↬240]: https://github.com/axodotdev/oranda/pull/240
[↬248]: https://github.com/axodotdev/oranda/pull/248


## 0.0.1 - 2023-02-24

Initial release.
