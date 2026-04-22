# How to import notes from Obsidian

NotePlan supports Markdown and your notes are saved locally on your devices. Obsidian manages notes in a similar way, so technically you can open NotePlan's folder as a vault in Obsidian and [use both apps simultaneously](61-use-noteplan-with-obsidian.md). However, there are some differences in how NotePlan and Obsidian manage notes. For example, NotePlan uses the first line in the note as the title while Obsidian uses the filename.

You can switch to using only NotePlan to avoid the problems of using the two apps simultaneously and take full advantage of NotePlans task management and calendar features.

## Steps

In the menubar (macOS version) under File > Import Notes... > Obsidian, you can migrate your Obsidian vault into NotePlan (the original notes won't be changed or moved, NotePlan makes a copy). In this process NotePlan will adapt the Markdown format (note titles, attachments, etc. read details below) to fit to how NotePlan manages the notes.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/65650901d36bda1a4759ae1b/file-36v4bsuGv1.png)

In the next step (after the help dialog), select the Obsidian vault you want to import.

## Calendar Notes

NotePlan will attempt to import calendar notes wherever they are in your vault and move them into NotePlan's Calendar folder outside the regular notes. The calendar notes have to follow the following format:

`YYYY-MM-DD.md` or `YYYYMMDD.md`.

The imported calendar notes will merge with existing calendar notes inside NotePlan (if any).

This works with daily, weekly, monthly, quarterly and yearly notes. Make sure to configure the calendar notes plugin of Obsidian, so that it follows this naming convention and that the calendar notes are in the root folder.

Weekly format: `YYYY-Www.md` (for example 2023-W02.md)
Monthly format: `YYYY-MM.md`
Quarterly format: `YYYY-Qq.md` (for example 2023-Q1.md)
Yearly format: `YYYY.md`

## Note Titles

Obsidian uses the filename as note title, whereas NotePlan will look for the first non-empty line in your note (except it has front matter) to determine the note title. For his reason Obsidian notes often have no title inside the note, so NotePlan will take the filename and add an H1 title at the beginning of the note (after the frontmatter).

Note: In v3.9.11 or newer, NotePlan will add the title to the frontmatter, if it can find any and if it doesn't already contain a "title" or "name" attribute. Also, it will check if the note already has a title that is identical with the filename (title in Obsidians sidebar).

## Images

NotePlan will scan the contents of your Obsidian notes for `![[image/path]]` links and convert them to `![title](note_attachments/image)` and copy the images into a sub folder for each note. The path of the image inside the square brackets needs to be reachable for NotePlan to find the images.

## Modification and Creation Dates

From v3.9.11 NotePlan will preserve the modification and creation dates of the file.

## Tips

- If you want to import your vault into a single folder instead of "spreading" the files into the root folder of NotePlan, you can add or select a folder above the vault.

*Last updated: 2023-11-30*
