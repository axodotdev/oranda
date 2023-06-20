# Tips and Tricks

## Hiding the Markdown title

Oranda breaks out your project's title into its own header, which can be annoying if you've started your own
README.md with something like this:

```markdown
# myprojectname

Blah blah blah etc
```

If you build your oranda site like this, the title will appear twice! oranda supports a special class called `oranda-hide`
that you can wrap your title (or whatever you don't want to appear on the page) with, like this:

```markdown
<div class="oranda-hide">

# myprojectname

</div>

Blah blah blah etc
```

Keep in mind the line breaks before and after the HTML, otherwise the Markdown parser may not function correctly.
