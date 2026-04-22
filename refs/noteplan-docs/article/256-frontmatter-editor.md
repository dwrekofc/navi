# Note Properties (aka Frontmatter) Editor

*Note: Available from v3.16.3, needs macOS 14+, *[release notes](https://noteplan.co/changelog/v3.16.3-frontmatter-editor)*.*

The Note Properties Editor lets you edit the meta data of notes, without having to deal with the raw frontmatter.

- [Properties in Calendar Notes](#calendar-notes)
- [Key and Value Suggestions](#suggestions)
- [Select a Note Icon](#note-icon)
- [Select a Background Color & Pattern](#background-color)
- [Copying Frontmatter](#copy)
- [Templating Frontmatter](#templating)

The properties are used in the Notes List and Cards views. You can also use the editor to add icons and change the background color of notes or add more meta data to your existing notes. A more advanced use is to setup templates that pre-populate your notes with meta data (or icons and colors). This allows you to group and filter notes in the [Notes Table](238-notes-table-and-other-views.md).

If you click into the key and value fields, a drop down will appear pre-populated with values it could find in other notes (except the ones in the Templates or Trash folders), so you don't need to type them out every time. As you type, it will filter the possible suggestions.

There are a few special keys that will be available right away:

- `icon` Lets you set the icon of the note, displayed in the sidebar and notes overview.
- `icon-color` Lets you choose the icon color.
- `icon-style` Lets you choose the appearance of the icon = regular, solid or light.
- `bg-color` Choose the background color of the note.
- `bg-color-dark` Choose the background color of the note in dark mode.

## Properties in Calendar Notes

By default they are hidden, because it's less common to use properties inside calendar notes. To show them you can open the note menu top right and hit "Add Properties".

## Key and Value Suggestions

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67d0d78bcf07c42ef1e96e72/file-obK95k28ZQ.png)

The Frontmatter Editor will show suggestions if click into a key or value field and filter the results by what you type. The suggestions are coming from the properties of existing notes.

It will search your whole database of notes except templates, trash or archive folder. If you want to see relevant suggestions, you will need to add them manually first to other notes.

## Select a Note Icon

Type `icon` into the key field and once you click into the value field an icon picker will show up. By typing a keyword you can search for an icon. Use `icon-color` to pick a color for your icon and `icon-style` to change the style to `solid` or `light` (`regular` is the default).

The icon will be visible in the left sidebar, the command bar when you are searching for notes and in the notes list when you click on a folder.

## Select a Background Color & Pattern

Type `bg-color` or `bg-color-dark` (for dark mode) into the key field and when you click into the value field, a color picker will show up. Scroll through the suggestions to find your favorite color or start typing the name.

Each color has a different brightness indicated by the number like `Amber 100` is brighter and weaker than `Amber 900` (darker orange-red tone).

Besides the color, you can also change the pattern using `bg-pattern` and change it to lined, squared, mini-squared or dotted.

## Copying Frontmatter

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67d0d5340d58fd48b3c802be/file-6C28uwpGP8.png)

On the right side of the "Properties" button you will find a menu button which contains the options to copy, paste and delete all properties. Select copy and it will (in raw frontmatter format) copy your properties, so you can open another note and paste them there.

The paste will overwrite existing properties. If you want to add the properties to another note instead of replacing them, you can also click on the icon left of the keys:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67d0d5a5cf07c42ef1e96e6e/file-2pLM8g3xpL.png)

## Templating Frontmatter

*Note: This might change in later releases to simplify the process.*

Templates can be added to regular notes and also pre-create some default properties, for example for project notes. To achieve this a template will need two types of properties. The regular properties at the top of the template and below that a frontmatter block that uses two hyphens (--) instead of three.

For example:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67d0d9d1f72ba57193bd21dd/file-1FjX74BtPB.png)You can copy the example template from here:

```
--
status: Backlog
type: Product
icon: hammer
icon-color: amber-600
bg-color: amber-100
bg-color-dark: amber-900
--
# New Project
<br>
```

Now you can insert this template into a new regular note:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67f137b4cfee38650144efb9/file-eoWTf5Avcr.png)

If the note is empty and has no frontmatter, the template's frontmatter will be inserted:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67f137758cf71f547ecbd96f/file-kPeDwIQMf2.png)

*Last updated: 2025-08-23*
