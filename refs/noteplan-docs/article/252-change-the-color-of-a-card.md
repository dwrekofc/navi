# Change the background color of a Note/Card/Row

*Note: Available from v3.16.1*

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6796adbbc22a0e2cb971d8ee/file-zWjuL5QENz.png)

You can change the background color of a note, and card (in the Card View), and row (in "List" style table) by adding a "bg-color" field to the [Frontmatter](237-frontmatter.md) for light mode and "bg-color-dark" for dark mode. As value use a hex color value or a Tailwind CSS background color name. Find a full list of colors [here](https://tailwindcss.com/docs/background-color).

For example the color red as hex code:

```
---
bg-color: #ff0000
bg-color-dark: #11ff0000
---
```

Or a tailwind color:

```
---
bg-color: bg-sky-100
bg-color-dark: bg-sky-950
---
```

**Note**: This is only supported by regular (project) note. Not by daily notes yet.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/679d428a47fe012ada074410/file-EzBTWiW7zl.gif)**Tip**: If you type the dashes and they convert to a long dash, type them very fast or turn off "Smart Dashes" in the "Edit" > "Substitutions" menu or use the Note Helpers plugin and run the command "/convert to frontmatter".

*Last updated: 2025-01-31*
