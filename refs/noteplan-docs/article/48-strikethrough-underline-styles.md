# Strikethrough and Underline Styles

Previous: [Customize Themes](44-customize-themes.md)

Tips: [Add ~~Strikethrough~~](45-extend-noteplans-markdown.md), and [Add ~Underline~](45-extend-noteplans-markdown.md)

---

You can modify the line thickness and pattern of the strikethrough and underline styles. The default value, which is a simple thin line, for both is:

```
"underlineStyle": 1,
"strikethroughStyle": 1
```

Values from `1-8` increase the thickness.

The next bit values that have an effect are: `1`...

+8: double (= `9`)

+ 256: patternDot (= `257`)

+ 512: patternDash (= `513`)

+ 1024: patternDashDotDot (= `1025`)

+ 8192: over line (= `8193`)

+32768: byWord (= `32769`)

*Last updated: 2021-07-19*
