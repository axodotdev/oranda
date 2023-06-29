# Changelog

## Unreleased

FIXME: write a proper changelog for the next release

* various fixes
* much faster
* mdbook integration change

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
