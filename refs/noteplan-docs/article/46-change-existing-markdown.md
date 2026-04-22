# Change Existing Markdown

Learn more about:

- [NotePlan's Default Regular Expressions](#noteplansregex)
- [Resolving Conflicts](#conflicts)
- [Example 1 → #### Heading 5](#ex1)
- [Example 2 → Different Colors for Done / Canceled / Scheduled](#ex2)

---

You can not only modify the style attributes of existing styles, but you can also change the default regex. NotePlan will first look if there is a regex defined in your theme file and then it will fall back to the internal regex file. See examples below.

## NotePlan's Default Regular Expressions

Below is a JSON file containing the internal, default regular expressions for the existing themes, which you can use to get an idea and modify styles:  [Download markdown-regex.json](https://drive.google.com/file/d/1L16QZ1487i0uVLuA-K3l0sEhWTq7p86r/view?usp=sharing)
## Resolving Conflicts

If you add a custom style it will be applied by default after the existing styles. It could overwrite an existing style and break the styling. To resolve this conflict, you can change the order in which the styles are applied. To control the order you can add a new key “orderedStyles” above “styles” which contains an ordered array of all style names. Include your custom style wherever needed to resolve the conflict.

```
"orderedStyles": ["title-mark1", "title-mark2", "title-mark3", "body", "quote-content", "bold", "bold-left-mark", "bold-right-mark", "italic", "italic-left-mark", "italic-right-mark", "boldItalic", "boldItalic-left-mark", "boldItalic-right-mark", "code", "code-left-backtick", "code-right-backtick", "special-char", "checked-todo-characters", "todo", "checked", "quote-mark", "tabbed", "link", "hashtag", "attag", "schedule-to-date-link", "done-date", "schedule-from-date-link", "note-title-link", "title1", "title2", "title3", "note-title-link"],
```

Here is the current order of the existing styles:

## Example 1 → ##### Heading 5

NotePlan supports by default headings up to `####`. So Heading 5 will look exactly like Heading 4. If you want to add support for Heading 5 and beyond, you can first modify the `title4` regex to limit it to Heading 4, then add your own Heading 5+ style with a regex. Look up what the current regex is in the `markdown-regex.json`, then modify it.

Here is how the end result could look like:
```
"title4": {
    "regex": "^\\h*(#### )(.*)",
    "matchPosition": 2,
    "isRevealOnCursorRange": true,
    "color": "#FFA759",
    "underlineStyle": 1,
    "size": 16
},

"title5": {
    "regex": "^\\h*(#####+ )(.*)",
    "matchPosition": 2,
    "isRevealOnCursorRange": true,
    "color": "#F0A759",
    "size": 14,
    "underlineStyle": 1
},

"title-mark5": {
   "regex": "^\\h*(#####+ )(.*)",
   "matchPosition": 1,
   "isMarkdownCharacter": true,
   "isHiddenWithoutCursor": true,
   "isRevealOnCursorRange": true,
   "color": "#40000000",
   "size": 14,
   "font": "AvenirNext-DemiBold",
},
```

`”title4”`s regex `"^\\h*(####+ )(.*)"` was modified to `"^\\h*(#### )(.*)"` ( the + was removed) and `”title5”` was added. If you want to use it, make sure to delete your existing `”title4"` style, then paste the above.

## Example 2 → Different Colors for Done / Canceled / Scheduled

Right now all three todo states have the same color (a transparent grey). But if you want to have different colors for each state, you can overwrite the `checked` style and add 2 more with a slightly different regex:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085f2908af76a714bfdac51/file-5xD1z9PNAj.png)

Here is the code:

```
"checked": {
    "regex": "(^\\h*[\\*\\-\\+]{1} |^\\h*[0-9]+[\\.\\)] )(\\[x\\] )(.*)",
    "matchPosition": 0,

    "color": "#88CBFFC6",
    "headIndent": 33        
},

"checked-canceled": {
    "regex": "(^\\h*[\\*\\-\\+]{1} |^\\h*[0-9]+[\\.\\)] )(\\[\\-\\] )(.*)",
    "matchPosition": 0,

    "color": "#44FFCCC6",
    "headIndent": 33,
    "strikethroughStyle": 1
},

"checked-scheduled": {
    "regex": "(^\\h*[\\*\\-\\+]{1} |^\\h*[0-9]+[\\.\\)] )(\\[\\>\\] )(.*)",
    "matchPosition": 0,

    "color": "#88CBCCFF",
    "headIndent": 33
},
```

---

**Next up: **[Create Custom Links →](47-create-custom-links.md)

**Jump to:**

- [Customize Themes](44-customize-themes.md)
- [Extend NotePlan's Markdown](45-extend-noteplans-markdown.md)
- [Change existing Markdown](46-change-existing-markdown.md) *(you're here)*
- [Create Custom Links](47-create-custom-links.md)

*Last updated: 2023-08-08*
