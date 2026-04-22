# Create Custom Links

You can use attributes to detect and display links. See the following example:

```
"quick-link": {
    "regex": ">([\\w\\-.]+)",
    "matchPosition": 1,
    "urlPosition": 1,
    "type": "noteLink",
    "prefix": "noteplan://x-callback-url/openNote?noteTitle="
},
```

This will detect text prefixed with `>` and create a clickable link, like `>mynote`. You need to define the `”type”` as `”noteLink”` so that NotePlan turns the text into a link.

A link has a URL. You can define which part of the matched string will be used as a URL using the regex group position `urlPosition`. In this case `”urlPosition”: 1` is used.

The ` ”prefix”` will prepend any string before the URL. In this case, we are using an X-Callback-Url, so the link can be opened inside NotePlan: `"noteplan://x-callback-url/openNote?noteTitle="`. You can also use any web address. The final URL of `>mynote` for example would be:

```
"noteplan://x-callback-url/openNote?noteTitle=mynote"
```

In this case, NotePlan would attempt to open a note with the title “mynote”. [Learn more about X-Callback-Urls here](49-x-callback-url-scheme.md).

## Open Search with Regular Expressions

Following will kick off in-app searches on a delimited `/search phrase/`, for example for `/meetings/` will open the search view in NotePlan and search for the text “meetings” in all your notes:

```
"note-search-link":  {
    "regex": "/(\\S[^/]*\\S)/",
    "matchPosition": 1,
    "urlPosition": 1,
    "type": "link",
    "prefix": "noteplan://x-callback-url/search?text="
},
```

---

**Next up:** [Changelog →](211-theme-changelog.md)

**Jump to:**

- [Customize Themes](44-customize-themes.md)
- [Extend NotePlan's Markdown](45-extend-noteplans-markdown.md)
- [Change existing Markdown](46-change-existing-markdown.md)
- [Create Custom Links](47-create-custom-links.md) *(you're here)*
- [Changelog](211-theme-changelog.md)

*Last updated: 2023-08-08*
