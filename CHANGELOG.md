# Changelog

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
