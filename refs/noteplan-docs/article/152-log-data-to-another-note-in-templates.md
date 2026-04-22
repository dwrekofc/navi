# Log Data to Another Note in Templates

If you are tracking your [habits](144-habit-tracking.md) or health information such as your sleep quality on a daily basis you want to capture this data somewhere. One place is the daily notes but they don't give you a good overview if you want to review your captured data. A better place is a dedicated note where you can capture each day in a new row and review everything with just a glance.

Automate the capture part using templates and a prompt, so you don't need to open the note where you log your data every day. Using a daily note template NotePlan can ask you every day to enter the information you want to log and append it automatically to another note.

Here's the complete template:

```
---
title: Daily Note w/ Sleep Score Capture
type: empty-note
documentation: https://help.noteplan.co/article/136-templates
---
<% const sleepNoteTitle = '😴 Sleep Score' -%>
<% prompt('sleepScore', 'Score how well did you sleep (1-9)?') -%>
## Primary Focus
- 

## Tasks
- 

<% var sleepNote = DataStore.projectNoteByTitle(sleepNoteTitle)[0] -%>
<% if(!sleepNote) { sleepNote = DataStore.projectNoteByFilename(DataStore.newNoteWithContent('# ' + sleepNoteTitle + '\n')) } -%>
<% const sleepLine = date.format("DD MMM, 'YY", Editor.title) + `:\t**${sleepScore}**` -%>
<% sleepNote.appendParagraph(sleepLine, 'text') -%>
```

Here is how the template works after you have added it:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62a20c9202b8fd3b945fbe10/file-k4Rl4sEbZs.gif)

### How to add the template?

This approach works on iOS and Mac:

1. Copy the template text above.
2. Open NotePlan and create a new template in the sidebar under "Smart Folders > Templates".
3. Select everything in the new empty template (CMD+A) and paste the template text from this article (so the old text gets deleted).
4. Go to an empty daily note and hit the "Insert Template" button.

*Last updated: 2025-05-18*
