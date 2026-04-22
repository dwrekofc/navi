# How to create a link without a backlink?

Sometimes you want to link to a note without creating another backlink entry in the references area at the top of the other note. There are multiple ways you can achieve that. Either by using the existing linking options or by extending your theme.

## Copy an URL to the note

You can do this right now by creating a URL to a note:
1. Click on the menu top right, then "Copy URL to Note".
2. Write the title of this new link into the editor.
3. Select the title and hit CMD+K.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/628fb00e8bf21a2e2b6d7d56/file-wrGU0uR9i9.gif)

## Add custom links to your theme

Normally you link to a note using 2x square brackets: `[[note name]]`, but this creates a backlink to the target note. You can also add a new way to link to a note with something else than square brackets and this won't create backlinks. For example `>note name<`:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6290fd2d92cb8c175b467abb/file-08bOEH6Xwn.gif)

Here is the code snippet you need to add to a [custom theme](44-customize-themes.md) (thanks to [@gracius on Discord](https://discord.com/channels/763107030223290449/810232612139827261/937081777644769280) for creating and sharing this):

```
"arrow-link": {
  "regex": ">((\\S[^<]*(?=<))|[\\S]+)",
  "matchPosition": 1,
  "urlPosition": 1,
  "backgroundColor": "#10000000",
  "isRevealOnCursorRange": true,
  "type": "noteLink",
  "prefix": "noteplan://x-callback-url/openNote?noteTitle="
},

"arrow-left-delimiter": {
  "regex": "(>)(\\S[^<]*)(<)",
  "matchPosition": 1,
  "isMarkdownCharacter": true,
  "isHiddenWithoutCursor": true,
  "isRevealOnCursorRange": true,
},

"arrow-right-delimiter": {
  "regex": "(>)(\\S[^<]*)(<)",
  "matchPosition": 3,
  "isMarkdownCharacter": true,
  "isHiddenWithoutCursor": true,
  "isRevealOnCursorRange": true,
}
			
```

Or download the complete custom theme containing this [here](https://drive.google.com/file/d/1sZ7BfePzJiJeEhBfCUBsIfKY43zzoBCJ/view?usp=sharing).

*Last updated: 2022-05-27*
