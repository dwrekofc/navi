# Customize Themes

Learn more about:

- [JSON File](#jsonfile)
- [Style Attributes](#styleattributes)
- [Link Colors](#linkcolors)
- [Fonts](#fonts)
- [Font Names](#fontnames)
- [Custom Fonts on iOS](#fontsios)
- [System Fonts](#systemfonts)
- [Editor](#editor)
- [Spacing](#spacing)
- [Head Indent](#headindent)
- [Tasks and Bullets](#tasksbullets)
- [Task States (Open, Scheduled and Canceled)](#taskstates)
- [Special Char](#specialchar)
- [Titles](#titles)
- [Code Fences](#codefences)
- [Building Custom Styles with Regular Expression](#customregex)
- [Changeglog](#changelog)

---

## How to Get Started

👉 [Watch the full walkthrough on YouTube](https://youtu.be/TE3YQ-Y2Ya0)
⬇️ [Official Theme Community Repository](https://github.com/brokosz/NotePlan_Themes)

Open NotePlan, click on “Preferences”, choose a theme as a template and click on “Copy & Customize”:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60859406e0324b5fdfd0e728/file-p4jAqGQJHY.png)

This will copy the theme JSON file into NotePlan’s synced folder structure from where you can modify it. Any changes you make on the theme will be automatically synced to all your other devices, including iOS.

You can also import other themes by clicking on “Import Theme...” in the same window.

*Note: If you change the custom theme, you will see the effects in NotePlan only if you switch to another theme and back to the custom theme.*

## JSON File

The theme file contains two major keys: “editor” and “styles”. The “editor” key defines the main colors of NotePlan's user interface:

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

(Note "timeBlockColor" is available from v3.3)

Optional:

- `"sidebarStyleOverride": "dark"` For overriding the style of the sidebar. This way you can have a dark sidebar while your system is in light mode (on Mac).
- `"sidebarIconColorOverride": "#ffffff"` For overwriting the color of all icons in the sidebar that have the text color (white or black mostly).
- `"sidebarFolderColorOverride": "#ff0000"` For overwriting the color of folder icons in the sidebar (v3.20.1).

The “styles” key contains all text styles, like font, text size, color, etc. for the titles, open tasks, closed tasks, quotes, etc.:

```
"styles": {
    "body": {
        "font": "AvenirNext-Regular",
        "size": 16,
        "color": "#333333",
    },
    "title1": {
        "color": "#000000",
        "size": 28
    },
    "title2": {
        "color": "#000000",
        "size": 24
    },
...
```

## Style Attributes

The following attributes are valid for style keys with example values:
```
"lineSpacing": 3,
"paragraphSpacing": 8,
"paragraphSpacingBefore": 8,
"color": "#000000",
"backgroundColor": "#FFFFFF",
"size": 24,
"type": "italic",
"underlineStyle": 1,
"underlineColor": "#000000",
"strikethroughStyle": 1,
"strikethroughColor": "#000000",
"leadingBorder": false // Shows a border left of the paragraph like quotes do
"borderRadius": 6, // Use with backgroundColor for a rounded background
"inlineBorder": false // When using borderRadius for inline words like highlighting 
"leftBorderPadding": 0, // Increases the width of the border on the left side
"rightBorderPadding": 0, // Increases the width of the border on the right side
"horizontalMargin": 0, // Adds margins to a word (gives space for the border)
"isFullWidthBorder": true // Expands the boarder to the full text width
```

Strikethrough and underline styles have [a thicker line the higher the number](48-strikethrough-underline-styles.md) you are using for the `...style` attribute. Adding color is optional.

## Link Colors

The color for following keys will be overwritten by `tintColor`, because they are all converted to links and only one link color can be used (limitation in Apple’s text editor framework):

```
"link"   
"schedule-to-date-link"
"done-date"
"schedule-from-date-link"
"note-title-link"
"hashtag"
"attag"
"phonenumber"
```

## Fonts

The font of most keys, including the following is overwritten by the app preferences under “Editor”, so that you can change the font and font-size from within the app without having to modify a theme:

```
"body"
"title1"
"title2"
"title3"
```

You can turn off the overwriting and define your own font by setting following attribute under the “editor” key in a theme file:

```
"shouldOverwriteFont": false
```

It’s set to `true` by default.

### Font Names

You can look up the correct font name using the app “Font Book”. Copy the “PostScript name” value into the theme file:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60859be4e0324b5fdfd0e74d/file-9zeMDor865.png)In the theme file change the value for “font” in the “body” key for example:

```
"body": {
    "font": "AvenirNext-Regular",
    "size": 16,
    "color": "#CBCCC6",
},
```

### Custom Fonts on iOS

On iOS, the font will revert to the System font if you choose a custom font on Mac that you have installed from the internet. To install custom fonts on iOS use an app like [FontCase](https://apps.apple.com/us/app/fontcase-manage-your-type/id1205074470) or [AnyFont](https://apps.apple.com/us/app/anyfont/id821560738). Follow the instructions when you open FontCase.

### System Fonts

If you want to use the system fonts, you can use the following values for body, italic and bold. It will automatically use the system’s font:
Bold: `.AppleSystemUIFontHeavy`
Italic: `.AppleSystemUIFontItalic `
Normal: `.AppleSystemUIFont`

## Editor

The theme files are written in JSON. If you miss one comma anywhere, it won’t load. The easiest way to reduce such time-wasting errors is to use an editor which supports JSON. For example “Sublime Text”. It will show you if you have missed anything:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085dafae0324b5fdfd0e7b9/file-B74HhYlfuh.png)

## Spacing

If you want to change the `paragraphSpacing,` `lineSpacing` or, `paragraphSpacingBefore` for the titles, you need to add the attributes to the corresponding `"title-markX"` keys instead of `”titleX”` keys.
By default the `paragraphSpacing` is different from the `lineSpacing`. In most apps these values are the same. If you want to have the same value for both, add them to the `”body”` key. Make sure to add it also to the `“tabbed”` key which styles anything that is indented.

### Head Indent

This defines the indentation of the first line vs the other lines inside a paragraph. There is a difference between the lines if the line is a task for example. In case of a task, there is a task icon in the first line and it shouldn’t indent along with the text, that’s what `headIndent` is achieving:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085d8c2e0324b5fdfd0e7b6/file-KQ5HuU2Xoj.png)

## Tasks and Bullets

Tasks and bullets are styled by the `”todo”` key. There is no separation because the preferences determine what a task is and what a bullet is. For example, `* task` is by default a task, but if you turn it off in the preferences, it becomes a bullet and `- task` can be a task or the standard markdown: `- [ ] task`.

### Tasks States (Open, Scheduled, and Canceled)

All state styles are covered under `”checked”`. In the background, a regular expression is defined to identify them in the text. They are bundled into one style to reduce the computing power required.

### Special Char

This refers to `*` and `-` which are special characters in NotePlan. By default, a different font is assigned to those two characters (Menlo-Regular) to make them look better. This font is not overwritten by the one defined in the preferences.

## Titles

`title1` covers headings with a single pound `# Heading`, `title2` with two pounds `## Heading 2` and `title3` covers anything else, from `### Heading 3` and more pounds. There is no support for` title4` or higher. Instead, you can add custom regular expressions. More about this below.

## Code Fences

You can change the theme of code fences and more. Here is a list of options with examples, which can be added to the styles:

```
"code-fence": {
   "theme": "github",
   "corner-radius": 8,

   "body": {
       "font": "Menlo-Regular",
       "backgroundColor": "#f5f5f5"
   },

   "language-specifier": {
       "font": "Menlo-Italic",
       "color": "#afafaf"
   },

   "fence-open": {
       "color": "#afafaf"
   },

   "fence-close": {
       "color": "#afafaf"
   }
}
```

Find a list of supported Code Fencing themes [here](85-code-fencing-themes.md), which you can use in the "theme" attribute above.

## Building Custom Styles with Regular Expression

Every style is based on a regular expression to detect the text. The regular expressions are stored in a JSON file inside the app bundle, but you can add custom regular expressions to the theme JSON file.

## Changelog

Find all changes to the themes [here](211-theme-changelog.md).

---

**Next up: **[Extend NotePlan's Markdown →](45-extend-noteplans-markdown.md)

**Jump to:**

- [Customize Themes](44-customize-themes.md) *(you're here)*
- [Extend NotePlan's Markdown](45-extend-noteplans-markdown.md)
- [Change existing Markdown](46-change-existing-markdown.md)
- [Create Custom Links](47-create-custom-links.md)
- [Changelog](211-theme-changelog.md)

*Last updated: 2026-01-18*
