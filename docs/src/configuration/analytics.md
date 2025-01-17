# Analytics

oranda supports automatically inserting the correct analytics snippet your provider into your generated pages.

Right now we support the following analytics providers:

- [Google Analytics](https://analytics.google.com/analytics/web/)
- [Plausible](https://plausible.io/)
- [Fathom](https://usefathom.com/)
- [Umami](https://umami.is/)

To add any of these, add the required configuration under the `analytics` key:

### Google Analytics

```json
{
  "marketing": {
    "analytics": {
      "google_analytics": {
        "tracking_id": "String"
      }
    }
  }
}
```

### Plausible

```json
{
  "marketing": {
    "analytics": {
      "plausible": {
        "domain": "String",
        "script_url": "Optional string for self hosted"
      }
    }
  }
}
```

### Fathom

```json
{
  "marketing": {
    "analytics": {
      "fathom": {
        "site": "String"
      }
    }
  }
}
```

### Umami

```json
{
  "marketing": {
    "analytics": {
      "umami": {
        "website": "String",
        "script_url": "String"
      }
    }
  }
}
```
