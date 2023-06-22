# Analytics

When it comes to analytics, oranda gives you a simple interface to add your scripts and make sure you collect analytics.

Right now we support the following analytics providers:

- [Google Analytics](https://analytics.google.com/analytics/web/)
- [Plausible](https://plausible.io/)
- [Fathom](https://usefathom.com/)
- [Unami](https://umami.is/)

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

### Unami

```json
{
  "marketing": {
    "analytics": {
      "unami": {
        "website": "String",
        "script_url": "String"
      }
    }
  }
}
```
