# Theming

## Predefined themes

Oranda comes with four default themes:

- Light
- Dark
- Axo Light (`axo_light` or `axolight`)
- Axo Dark (`axo_dark` or `axodark`)
- Hacker
- Cupcake

You can change the theme by adding the `styles.theme` key to your `oranda.json`:

```json
{
  "styles": {
    "theme": "hacker"
  }
}
```

Dark is the default theme.

## Customizing Themes

Themes can be further customized by adding extra CSS.

Additional CSS can be added using the `styles.additional_css` key.

```json
{
  "styles": {
    "additional_css": ["./local/file.css", "http://www.remote.dev/file.css"]
  }
}
```

> Note: Remote files will be copied and the copy served locally, so once a link is updated, the site must be regenerated for changes to take effect.

### Adding CSS

Oranda's CSS makes use of [cascade layers](https://css-tricks.com/css-cascade-layers/) to scope CSS and make it simpler to override styles. To override themed styles, say on a `<p>` element, place it inside a layer called `overrides`.

```css
@layer overrides {
  p {
    color: aquamarine;
  }
}
```

Alternately, CSS that is not defined within a layer has precedence over all layered CSS, so this will also work.

```css
p {
  color: aquamarine;
}
```

### Dark vs. Light

When the `dark` theme is selected, a `dark` class is added to the page, and styles to be applied in dark mode only can include this selector. For instance,

```css
.dark p {
  color: aquamarine;
}
```

Will create paragraphs colored aquamarine in dark mode only.

### Adding Classes

When there are specific elements you would like to add to your pages, these can be added into Markdown files as raw HTML with class selectors that you can target with your CSS.

```html
<!-- README.md -->

## A Different Kind of Box

<div class="my-border-class">
  <p>An outlined box</p>
</div>
```

```css
.my-border-class {
  padding: 1rem;
  border: 6px dotted seagreen;
}
```

## Creating a New Theme

Currently, to create a new theme, you need to follow the directions above in "Customizing Themes" and overwrite the given CSS. We recommend continuing the layer approach and placing overrides in the `overrides` layer and then adding a new named layer for your theme.

The ability to add a different theme directly will be included in future releases. Following the layers approach will make it simpler to transition your theme.
