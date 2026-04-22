# Theme Changelog

In this article you can read about the changes made to themes over time, so you can update your custom themes as needed.

## v3.9.4

In this release, we have introduced rounded background borders, and there is also a list of new styles. You can look up and copy these styles also by customizing an existing theme in v3.9.4. All themes have been upgraded with these styles. Hashtags and mentions also got a rounded "pill shaped" background.

#### Styles

1. `flagged-1`, `flagged-2`, and `flagged-3` to style tasks or bullets that have a leading "!" - "!!!" with a red background to signal the priority.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/64d259579d8cf153a015dab7/file-27GebeCiKd.png)

Add this to your theme to support it (under "styles"):

```
"flagged-1": {
     "backgroundColor": "#22D04023",
     "color": "#59000B"
},

"flagged-2": {
    "backgroundColor": "#40E3021D",
     "color": "#420008"
},

"flagged-3": {
    "backgroundColor": "#6FFF0202",
    "color": "#1F0004"
},
```

2. `working-on` to style tasks or bullets that have a leading ">>" with a purple background to signal that you are working on this task right now.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/64d259bd2f9efa0a1b131df5/file-NOL085Gips.png)

Add this to your theme to support it (under "styles"):

```
"working-on": {
     "backgroundColor": "#306600FF",
     "color": "#17003B"
},
```

3. `highlighted`, `strikethrough`, and `underline`

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/64d259dd17822902c5ab908e/file-VvX4xn5zfF.png)

Add this to your theme to support it (under "styles"):

```
"highlighted": {
    "backgroundColor": "#6684FF00",
},

"highlighted-left-marker": {
    "color": "#40000000",
},

"highlighted-right-marker": {
     "color": "#40000000",
},

"strikethrough": {
     "color": "#99333333",
     "strikethroughColor": "#DD333333"
},

"strikethrough-left-tilde": {
     "color": "#44333333"
},

"strikethrough-right-tilde": {
     "color": "#44333333"
},

"underline": {
     "underlineStyle": 2,
     "underlineColor": "#FFCC66"
},

"underline-left-tilde": {
    "color": "#44333333"
},

"underline-right-tilde": {
    "color": "#44333333"
}
```

#### Key/Value Attributes

This is a list of the new attributes and their explanation:

```
"leadingBorder": false // Shows a border left of the paragraph like quotes do
"borderRadius": 6, // Use with backgroundColor for a rounded background
"inlineBorder": false // When using borderRadius for inline words like highlighting 
"leftBorderPadding": 0, // Increases the width of the border on the left side
"rightBorderPadding": 0, // Increases the width of the border on the right side
"horizontalMargin": 0, // Adds margins to a word (gives space for the border)
"isFullWidthBorder": true // Expands the boarder to the full text width
```

#### Examples

Here are a few examples of how you can use the new attributes. These examples are from the base theme which enables highlighting, strikethrough, etc to all themes. This is the highlighted style:

```
"highlighted": {
    "regex": "(==)([^=\\`]{1,})(==)",
    "matchPosition": 0,
    "isRevealOnCursorRange": true,
    "borderRadius": 6,
    "horizontalMargin": 3,
    "leftBorderPadding": 3,
    "rightBorderPadding": 0,
    "inlineBorder": true,
},

"highlighted-left-marker": {
    "regex": "(==)([^=\\`]{1,})(==)",
    "matchPosition": 1,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
},

"highlighted-right-marker": {
    "regex": "(==)([^=\\`]{1,})(==)",
    "matchPosition": 3,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
},
```

The following is the flagged-1 style:

```
"flagged-1": {
    "regex": "(^|\\v)(\\h*(\\*|-|\\+)\\h*(\\[ \\]\\h+|\\h+)\\!{1}\\h+.*)($|\\v)",
    "matchPosition": 2,
    "borderRadius": 5,
    "leftBorderPadding": 4,
    "rightBorderPadding": 4,
    "matchLine": true,
},
```

## v3.3

The new body attribute `timeBlockColor`, was added under "editor" to change the color of timeblocks in the editor and in the timeline on the right (bottom on iPhone). Example:

```
"editor": {
    "backgroundColor": "#ffffff",
    "altBackgroundColor": "#FAFFFF",
    "tintColor": "#d87001",
    "tintColor2": "#0091f8",
    "textColor":"#333333",
    "toolbarBackgroundColor": "#F2F3F5",
    "toolbarIconColor": "#444444",
    "menuItemColor": "#444444",
    "timeBlockColor": "#d87001",
    "shouldOverwriteFont": true
},
```

---

**Jump to:**

- [Customize Themes](44-customize-themes.md)
- [Extend NotePlan's Markdown](45-extend-noteplans-markdown.md)
- [Change existing Markdown](46-change-existing-markdown.md)
- [Create Custom Links](47-create-custom-links.md)
- [Changelog](211-theme-changelog.md) *(you're here)*

*Last updated: 2023-08-08*
