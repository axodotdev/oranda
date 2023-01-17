# Analytics

When it comes to analytics Oranda gives you a simple interface to add your scripts and make sure you collect analytics.

Right now we support the following analytics providers:

- [Google Analytics](https://analytics.google.com/analytics/web/)
- [Plausible](https://plausible.io/)
- [Fathom](https://usefathom.com/)
- [Unami](https://umami.is/)

To add any of these you can add to your `oranda.json` the info for the one you use:

### Google Analytics

```json
{
  "analytics": {
    "google_analytics": {
      "tracking_id": "String"
    }
  }
}
```

### Plausible

```json
{
  "analytics": {
    "plausible": {
      "domain": "String",
      "script_url": "Optional string for self hosted"
    }
  }
}
```

### Fathom

```json
{
  "analytics": {
    "fathom": {
      "site": "String"
    }
  }
}
```

### Unami

```json
{
  "analytics": {
    "unami": {
      "website": "String",
      "script_url": "String"
    }
  }
}
```
