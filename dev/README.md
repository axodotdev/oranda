<!-- This is a comment and doesn't appear -->

> This README contains expected Markdown elements. It is useful for developing themes and testing that changes to Oranda parsing do not have unexpected repercussions. If you run into a bug not captured here, please add it.

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

A paragraph can contain **bold**, _italic_ and *italic* text. (Those two are different.) Let's look at some longer text and other features.

## A combination of text and headings, plus a blockquote

Do you know the poet Frank O'Hara?

He's one of my favorites!

He is a modern poet, active in the middle of the twentieth century. He worked as a guard at the Museum of Modern Art in New York as his day job. He was close friends with Abstract Expressionist painter Grace Hartigan. I thought it was Helen Frankenthaler but I found out I was incorrect. Both are wonderful painters and colorists.

But back to Frank. My favorite poem of his is called ["The Day Lady Died,"](https://www.poetryfoundation.org/poems/42657/the-day-lady-died) and is about the death of Billie Holiday. But my favorite quotation comes from "Meditations in an Emergency":

>  However, I have never clogged myself with the praises of pastoral life, nor with nostalgia for an innocent past of perverted acts in pastures. No. One need never leave the confines of New York to get all the greenery one wishes—I can’t even enjoy a blade of grass unless I know there’s a subway handy, or a record store or some other sign that people do not totally regret life. It is more important to affirm the least sincere; the clouds get enough attention as it is and even they continue to pass. Do they know what they’re missing? Uh huh.

The above text includes a:

> blockquote

and a [link](#).

But it does not include `inline code` or a fenced block with syntax highlighting

```js
const poets = ["O'Hara", "Millay", "Ginsberg", "Nash", "Eliot"];

poets.forEach((poet) => {
  console.log(`${poet}? I loooove ${poet}!`);
});
```

One error we ran into before was in code-blocks with HTML specifically, so let's add one of those, just for fun:

```html
<meta name="description" content="Poets, poets, poets" />
<meta property="robots" content="robots.txt" />
```
## Complex features: Images, tables, raw html


### Images
![An image from a vintage video synth](./static/some_art.png)
Here is an image I made.

Here is one with an external link
![](https://images.pexels.com/photos/14900787/pexels-photo-14900787.jpeg)

### Tables
I couldn't think of a good table example, so I adapted [this styles table from Github](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax).

| Style | Syntax | Example | Output |
| --- | --- | --- | --- |
| Bold | `** **` or `__ __` | `**Frank O'Hara is a bold poet.**` | **Frank O'Hara is a bold poet.** |
| Italic | `* *` or `_ _`   | `*I feel a little tipsy.*` | *I feel a little tipsy.* |
| Strikethrough | `~~ ~~` | `~~This task has been completed.~~` | ~~This task has been completed.~~ |
| Bold and nested italic | `** **` and `_ _` | `**I can be bold and also _tipsy_!**` | **I can be bold and also _tipsy_!** |
| All bold and italic | `*** ***` | `***Bold. Tipsy. Me.***` | ***Bold. Tipsy. Me.*** |
| Subscript | `<sub> </sub>` | `<sub>I am small and low.</sub>` | <sub>I am small and low.</sub> |
| Superscript | `<sup> </sup>` | `<sup>I am small and high.</sup>` | <sup>I am small and high.</sup> |

### Raw HTML

Don't forget that Markdown also accepts raw html (as we can see in the last few rows of the table above.)

<div class="title">For instance, this is a plain div with a class attached to style it in the title style.<sup>Wow.</sup></div>

You can also have things in your markdown that will not show on oranda by adding the class `oranda-hide`.

<div class="oranda-hide">
    you can't see this
</div>

## Nesting

### Lists

#### Unordered
+ We can go
    + Down
        + Down
- Using
    + Different
        * Prefixes


#### Ordered
1. The prefixes
    1. are ordered
        1. and subdivide
    1. down and then up
1. Once more


## Footnotes
Add a note with a number[^1].
Or a name [^psst].

[^1]: Number 1.
[^psst]: A secret.
