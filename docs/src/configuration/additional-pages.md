# Additional Pages

If you have extra Markdown files you'd like to link directly as root pages on your generated website, you can
use the `additional_pages` option to list them.

The option's format is an object with the human-readable page name as keys, and the path to the file as values. Example:

```json
{
  "additional_pages": {
    "Another page": "./AnotherFile.md"
  }
}
```
