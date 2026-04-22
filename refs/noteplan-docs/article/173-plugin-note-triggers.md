# Plugin Note Triggers

Note: Available in v3.7.2

Note Triggers can call a command of a plugin upon specific events, such as when a note will be saved. You can register a trigger for a specific note by adding the event and command as frontmatter at the top of a note.

## FrontMatter

Here is a simple example of a complete note with frontmatter:

```
---
triggers: onEditorWillSave => np.test.onEditorWillSave
---

# Note Title
text
```

The syntax: `triggers: onEditorWillSave => plugin.id.commandName`. In this case, the plugin has the ID `np.test` and the command has the name `onEditorWillSave` (note, this is the `name` attribute in the plugin.json, not the `jsFunction`).

## plugin.json

The plugin command has to be added to the `plugin.json` as an available command. You can add the attribute `"hidden": true` to hide it from the command bar and make it only available to the triggers. You can add more commands by separating them with commas (`onEditorWillSave => plugin.id.commandName, onEditorWillSave => plugin.id2.commandName2`, etc.).

```
{
   "noteplan.minAppVersion": "3.7.2",
   "plugin.id": "np.test",
   "plugin.name": "TEST",
   "plugin.description": "",
   "plugin.author": "NotePlan",
   "plugin.version": "0.5",
   "plugin.script": "script.js",
   "plugin.commands": [
     {
	"name": "onEditorWillSave",
	"description": "Do something when a note is edited",
	"jsFunction": "onEditorWillSave",
	"hidden": true,
	"arguments": []
     }		
    ]
}
```

In this example the command `name` and the `jsFunction` are identical. They don't have to be. However, it's recommended to make the `name` simple, so it's easier to add it to the FrontMatter.

## JavaScript Code (script.js)

The JavaScript function of the command will be called whenever the event triggers. In this case when a note has been edited and is about to be saved (there can be a few seconds of delay). In this example, we are changing the contents of the paragraph with index 4:

```
async function onEditorWillSave() {
  var paragraphs = Editor.paragraphs
  console.log("the paragraphs: " + paragraphs.length)
  paragraphs[4].content = "test"
  Editor.updateParagraphs(paragraphs)
}
```

Note that the command gets only triggered for the note with this frontmatter entry.

## Available Triggers

- `onEditorWillSave`: The note has been edited and NotePlan is about to save the changes to the file. Make changes here using the Editor object. These changes will be saved to the file as well. Don't use the Note object to make changes, they will be overwritten by what's currently visible in the Editor.
- `onOpen`: The note has been opened (the Note object of the opened note will be passed to the function)

## Demo

In this demo, we are changing one paragraph when the note has been edited.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6397362eefdd5760d1d80a06/file-VcbtkMthkq.gif)

## Note Diffs

To find out what has changed inside the note (so you can make informed changes to the note if needed), you can access all versions of that note and compare them against the current content. Access the `.versions` attribute of the note object that was passed to the JavaScript function. Each version has a `.content` and `.date` attribute. The first entry is the current version and the second one contains the previous content:

```
async function onEditorWillSave() {
  const note = Editor.note
  console.log("\n")
  console.log("onEdit triggered, note: '" + note.filename + "'")
  console.log("previous version: \n" + note.versions[1].content)
  console.log("\n")
}
```

Use `NotePlan.stringDiff(version1, version2)` to get an array of changed ranges:

```
async function onEditorWillSave() {
  const note = Editor.note

  console.log("\n")
  console.log("onEdit triggered, note: '" + note.filename + "'")

  // Get changed ranges
  const ranges = NotePlan.stringDiff(note.versions[1].content, note.versions[0].content)
  console.log("Changed content from index: " + ranges[0].start + " to: " + ranges[0].end)
  console.log("\n")
}
```

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6378e8bb3fc88c6e0f006fcd/file-x24XcFdZRT.gif)

*Last updated: 2023-05-11*
