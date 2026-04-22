# Extend NotePlan's Markdown

Learn more about:

- [Example 1 → ::highlighting::](#ex1)
- [Example 2 → !! flagging by importance](#ex2)
- [Example 3 → ~~Strikethrough~~](#ex3)
- [Example 4 → ~Underline~](#ex4)
- [Regular Expression Attributes](#regexattr)

---

*Note: The examples you see below have been implemented into all themes from v3.9.4. The code in this article serves as example to learn how to create your personal custom markdown.*

You can extend NotePlan‘s Markdown in the same JSON files used for customizing themes by adding a few extra attributes for the regular expressions. The following examples demonstrate this. Get started with [theme customization](44-customize-themes.md) before continuing here.

**Note:** While a regular expression is applied across the full text range of a note (not paragraph by paragraph), it will break if applied to multiple paragraphs (= text separated by newline characters), when the lines are being edited. NotePlan doesn't re-apply the styles to the full text when just one paragraph was changed. To save resources, only the edited paragraph is refreshed.

## Example 1 → ::highlighting::

Here is an example for a new style you can add below the existing ones for highlighting text by surrounding words with a double colon:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085db968996210f18bd6dc1/file-Z1BEo8UHpV.png)

Code:

```
"highlighted": {
    "regex": "(::)(.+?)(::)",
    "matchPosition": 2,
    "isRevealOnCursorRange": true,
    "backgroundColor": "#7745A2E5"
},
"highlighted-left-colon": {
    "regex": "(::)(.+?)(::)",
    "matchPosition": 1,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
    "color": "#AA45A2E5",
    "backgroundColor": "#7745A2E5",
},
"highlighted-right-colon": {
    "regex": "(::)(.+?)(::)",
    "matchPosition": 3,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
    "color": "#AA45A2E5",
    "backgroundColor": "#7745A2E5",
}
```

You can also overwrite the regular expression of an existing style by adding the `”regex”` attribute.

Here the keys `highlighted`, `highlighted-left-colon`, and `highlighted-right-colon` were added. They all have an `”regex”` attribute with the value `"(::)(.+?)(::)"` which will find words surrounded by `::` and apply the style defined in the same key.

## Example 2 → !! flagging by importance

In this example, you can change the background color of the full line from a light to a strong red tone to mark the priority using `!`, `!!` or `!!!` at the beginning of the line:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085dba4f8c0ef2d98df5f52/file-iJHOodgNUg.png)

Code:

```
"flagged-1": {
    "regex": "(^|\\v)(\\h*(\\*|-|\\+)\\h*(\\[ \\]\\h+|\\h+)\\!{1}\\h+.*)($|\\v)",
    "matchPosition": 2,
    "backgroundColor": "#22D0021B",
    "headIndent": 33
},
"flagged-2": {
    "regex": "(^|\\v)(\\h*(\\*|-|\\+)\\h*(\\[ \\]\\h+|\\h+)\\!{2}\\h+.*)($|\\v)",
    "matchPosition": 2,
    "backgroundColor": "#55D0021B",
    "headIndent": 33
},
"flagged-3": {
    "regex": "(^|\\v)(\\h*(\\*|-|\\+)\\h*(\\[ \\]\\h+|\\h+)\\!{3}\\h+.*)($|\\v)",
    "matchPosition": 2,
    "backgroundColor": "#AAD0021B",
    "headIndent": 33
},
```

## Example 3 → ~~Strikethrough~~

Add strikethrough style to a word using the following styles. This works together with an underline where you use single tilde characters left and right of the word:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085dc4b3149d33a19c70453/file-1eiHHSCYKY.png)

With visible markdown characters:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085dc528af76a714bfdac35/file-RxzBD59LX4.png)

Code:

```
"strikethrough": {
    "regex": "(^|[\\W_])(?:(?!\\1)|(?=^))((\\~|_)\\3)(?=\\S)(.*?\\S)(\\3\\3)(?!\\2)(?=[\\W_]|$)",
    "matchPosition": 4,
    "strikethroughStyle": 1,
    "isRevealOnCursorRange": true,
    "color": "#AACBCCC6",
    "strikethroughColor": "#CBCCC6"
},
"strikethrough-left-tilde": {
    "regex": "(^|[\\W_])(?:(?!\\1)|(?=^))((\\~|_)\\3)(?=\\S)(.*?\\S)(\\3\\3)(?!\\2)(?=[\\W_]|$)",
    "matchPosition": 2,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
    "color": "#44CBCCC6"
},
"strikethrough-right-tilde": {
    "regex": "(^|[\\W_])(?:(?!\\1)|(?=^))(\\~|_)\\2(?=\\S)(.*?\\S)(\\2\\2)(?!\\2)(?=[\\W_]|$)",
    "matchPosition": 4,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
    "color": "#44CBCCC6"
},
```

Looking for more strikethrough line thicknesses or patterns? [Continue here](48-strikethrough-underline-styles.md).

## Example 4 → ~Underline~

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085dd2f3149d33a19c70454/file-oknlWGMav3.png)

With visible markdown characters:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6085dd368af76a714bfdac38/file-7Sq4UQK69v.png)

Code:

```
"underline": {
    "regex": "(^|[\\W_])(?:(?!\\1)|(?=^))(\\~|_)(?=\\S)((?:(?!\\2).)*?\\S)(\\2)(?!\\2)(?=[\\W_]|$)",
    "matchPosition": 3,
    "isRevealOnCursorRange": true,
    "underlineStyle": 1,
    "underlineColor": "#FFCC66"
},
"underline-left-tilde": {
    "regex": "(^|[\\W_])(?:(?!\\1)|(?=^))(\\~|_)(?=\\S)((?:(?!\\2).)*?\\S)(\\2)(?!\\2)(?=[\\W_]|$)",
    "matchPosition": 2,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
    "color": "#44CBCCC6"
},
"underline-right-tilde": {
    "regex": "(^|[\\W_])(?:(?!\\1)|(?=^))(\\~|_)(?=\\S)((?:(?!\\2).)*?\\S)(\\2)(?!\\2)(?=[\\W_]|$)",
    "matchPosition": 4,
    "isMarkdownCharacter": true,
    "isHiddenWithoutCursor": true,
    "isRevealOnCursorRange": true,
    "color": "#44CBCCC6"
}
```

Looking for more underline line thicknesses or patterns? [Continue here](48-strikethrough-underline-styles.md).

## Regular Expression Attributes

`"matchPosition"` defines which regular expression group should be used for the styling. In this example every the regular expression has 3 groups, for example: `(::)`. The count begins with 1. If you use 0 as position, it takes the full match including all groups.

`"isHiddenWithoutCursor"` will hide the matched text if the cursor is not inside the word. `"isRevealOnCursorRange"` will reveal hidden text, if the cursor is inside.

`\\` escapes a character in the regular expression which would otherwise alter the matching behavior, such as `*`. Make sure it’s always double escaped like in this example.

---

**Next up: **[Change existing Markdown →](46-change-existing-markdown.md)

**Jump to:**

- [Customize Themes](44-customize-themes.md)
- [Extend NotePlan's Markdown](45-extend-noteplans-markdown.md) *(you're here)*
- [Change existing Markdown](46-change-existing-markdown.md)
- [Create Custom Links](47-create-custom-links.md)
- [Changelog](211-theme-changelog.md)

*Last updated: 2023-08-08*
