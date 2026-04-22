# Note Properties (aka Frontmatter)

Note properties are simple "key: value" pairs, such as "Status: Active", which give your notes metadata. This metadata is used across NotePlan in various places.

- [Defining the note title as property](#defining-the-note-title-as-property)
- [How to add Note Properties with a Template?](#how-to-add-note-properties-with-a-template)
- [How to create a new note with a Frontmatter Template?](#how-to-create-a-new-note-with-a-frontmatter-template)

**How to edit Note Properties?**
You can use the [Note Properties Editor](256-frontmatter-editor.md) at the top of your notes. In daily notes, the properties are hidden by default. Open the menu top right to "Add Properties".

**What is Frontmatter?
**Frontmatter is where your note's properties are stored in raw text format.

These properties are saved directly in the note's text file, enclosed in a special section called frontmatter. When the note is opened, the frontmatter is automatically converted into the [property editor](256-frontmatter-editor.md). This section must begin with a line containing three dashes (---) at the very start of the note and end with another line of three dashes. Everything between these two lines consists of key-value pairs that define your note's properties.

**How are Note Properties used?
**NotePlan uses properties for the [Notes Views/Table](238-notes-table-and-other-views.md), [Card View](239-card-kanban-view.md), [Auto-Templates](229-auto-insert-templates.md) and [Meeting Notes](134-meeting-notes.md). Also, some Plugins are utilizing the Frontmatter. Additionally, you can customize your notes, like assigning an icon or background color.

For example, the Notes Table allows you to group, filter and sort your notes by the Frontmatter keys and values it can find in your notes. Therefore, it's recommended to equip your notes with Frontmatter, if you want to visualize them later in different ways.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678d96bf797d2476b84e2998/file-VmBG4XJqKd.png)

## Defining the note title as property

NotePlan uses normally the first non-empty line of a note to determine the note's title. If you are defining it in the properties, NotePlan will ignore the title inside the note and use the one in the properties. A title property has the fixed name "title".

If no title is defined in the properties:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67ed65c46d642372f70e33dd/file-xiLHvH8mLX.png)

If a title is defined in the properties:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67ed66511a83f82fba7180a2/file-TzaOyUmsqE.png)

## How to add Note Properties with a Template?

*Note: This might change in later releases to simplify the process.*

Templates can be added to regular notes and also pre-create some default properties, for example for project notes. To achieve this a template will need two types of properties. The regular properties at the top of the template (added by default) and below that a raw frontmatter block that uses two hyphens (--) instead of three.

For example:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67ed683497f8701aacdbb29b/file-3WCtwJGQzH.png)

**Problem**: Typing "--" turns to a long dash
If you have "Smart Dashes" (macOS) or "Smart Punctuation" (iOS) enabled and try to type two dashes, the editor will turn them into a long dash.

*macOS*: Turn this off in the menubar under "Edit" > "Substitutions" > "Smart Dashes".
*iOS: *In the Settings app (system settings), search for "Keyboards" and turn off "Smart Punctuation"

## How to create a new note with a Frontmatter Template?

You can also create a new note directly with your Template to add the Frontmatter right away. Hit CMD+J (or the magnifier glass icon bottom right on iOS), then type "/new" and select "/Templating: Create new note using template"

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67f136798cf71f547ecbd96d/file-RRR8MEcv16.gif)

*Last updated: 2025-04-05*
