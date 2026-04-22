# Hidden Comments

If you add a lot of details to your tasks your note can get cluttered quite fast. The following theme can help by hiding blocks of text where you can add information about a task. Behind the task text (or any kind of paragraph) type your comment enclosed in `%%`, for example, `%%comment%%`.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62f696c780fd5a31e7ad1d06/file-0svxTiPmRZ.gif)

## Theme Code

If you have an existing [custom theme](44-customize-themes.md), you can add the following to your theme file to support this feature:

```
"comment": {
   "regex": "(%%)([^:]{1,})(%%)",
   "matchPosition": 2,
   "isRevealOnCursorRange": true,
   "isHiddenWithoutCursor": true,
   "color": "#888888"
},

"comment-left-colon": {
   "regex": "(%%)([^:]{1,})(%%)",
   "matchPosition": 1,
   "isMarkdownCharacter": true,
   "color": "#88818475"
},

"comment-right-colon": {
   "regex": "(%%)([^:]{1,})(%%)",
   "matchPosition": 3,
   "isMarkdownCharacter": true,
   "isHiddenWithoutCursor": true,
   "isRevealOnCursorRange": true,
   "color": "#88818475"
},
```

Or download the full theme file [here](https://gist.github.com/EduardMe/2225612c457651c13f2ef418017e8231/archive/dddaa5a495ef71128d4fe087e543021dccebd69f.zip).

You can import a theme on Mac using the menubar:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62f6989d80fd5a31e7ad1d0d/file-KhIvH29vDO.png)

This will be automatically synced to your iOS devices. If you are not using a Mac, you can open the Files app on iOS and navigate to "On my iPhone/iPad" > "NotePlan" > "Themes" and move the file to this folder.

## Contributions

Thanks to [@antony.sklyar](https://discord.com/channels/763107030223290449/963950027946999828/1003529048032555059) for sharing the theme code on [Discord](https://discord.gg/D4268MT).

*Last updated: 2022-08-12*
