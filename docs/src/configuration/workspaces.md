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
    // same as above
  },
  "styles": {
    "theme": "hacker" // set every site's theme to hacker
  }
}
```

Individual workspace member configs will still override what's set here, though. Also, _every_ key will be passed down,
including ones that don't make a lot of sense to be the same in multiple projects (for example [package manager](artifacts.md)
configuration).

Building a workspace will also generate a nice workspace index page that can be used to provide an overview over the
workspace's members, as well as some quick info and metadata.

## List of workspace configuration keys

- [name](#name) - set the overarching workspace name
- [members](#members) - list the workspace members
  - [members.slug](#membersslug) - the URL-safe slug to be used for this member
  - [members.path](#memberspath) - the path to this member's page source

### name

> Added in version 0.3.0.

Set the overarching workspace name. This is optional, and will fall back to "My Oranda Workspace" if not set (not very
intuitive, I know).

### members

> Added in version 0.3.0.

An array of objects representing the workspace members.

#### members.slug

> Added in version 0.3.0.

The URL-safe slug this page will be built at. This needs to be something that can be parsed as a URL, as well as a folder
name on your target system (because oranda is a static site generator, after all).

#### members.path

> Added in version 0.3.0.

The path to the page source. Point this to the same directory that the `oranda.json` would be in.