# Workspaces

oranda supports building multiple sites at once (referred to as building in a "workspace"). To control this behavior,
you can create a `oranda-workspace.json` file inside your workspace root. Running an oranda command will pick up this
file, and build the workspace members accordingly.

The reason why this is a separate file, and not part of the `oranda.json` file is to avoid confusing between _nonvirtual_
workspace root members (meaning if a workspace root also contains a site/package of some kind). By putting your workspace
configuration in a separate file, you can still have an oranda site at the same directory level, without any problems.

> **NOTE**: Workspace functionality will not be enabled if the `oranda-workspace.json` file doesn't exist!

A workspace configuration file looks something like this:

```json
{
  "workspace": {
    "name": "My Workspace",
    "members": [
      {
        "slug": "projectone",
        "path": "./project-one"
      },
      {
        "slug": "project_two",
        "path": "./project-two"
      }
    ]
  }
}
```

When ran with `oranda build`, this will produce two oranda sites, one at `/projectone`, and one at `/project_two`. oranda
will consider each separate project's `oranda.json` file (should it exist).

You can additionally pass down keys you'd like to be set for each member project:

```json
{
  "workspace": {
    "name": "My Workspace",
    "members": [
      {
        "slug": "projectone",
        "path": "./project-one"
      },
      {
        "slug": "project_two",
        "path": "./project-two"
      }
    ]
  },
  "styles": {
    "theme": "hacker"
  }
}
```

Individual workspace member configs will still override what's set here, though. Also, _every_ key will be passed down,
including ones that don't make a lot of sense to be the same in multiple projects (for example [package manager](artifacts.md)
configuration).

Building a workspace will also generate a nice workspace index page that can be used to provide an overview over the
workspace's members, as well as some quick info and metadata.
