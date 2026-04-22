# X-Callback-Url Scheme

In this article:

- [/openNote](#opennote)
- [/openView](#openview)
- [/addText](#addtext)
- [/addNote](#addnote)
- [/deleteNote](#deletenote)
- [/selectTag](#selecttag)
- [/search](#search)
- [/runPlugin](#runplugin)
- [/installPlugin](#installplugin)
- [/toggleSidebar ](#togglesidebar)
- [/noteInfo (x-success)](#noteinfo)
- [x-success](#xsuccess)
- [Testing](#testing)

---

General scheme:

`noteplan://x-callback-url/[action]?[action parameters]&[x-callback parameters]`

## /openNote

Open a note identified by the title or date.

**Parameters:**

- **noteDate** optional, to identify the calendar note in the format YYYYMMDD like '20180122' or use 'today', 'yesterday', 'tomorrow' instead of a date. To open weeks, use the ISO format YYYY-Www, for example, '2022-W32'.
- **timeframe** optional, if you are opening a calendar note, you can tell NotePlan here which type it should open: "week", "month", "quarter" or "year"
- **noteTitle** optional, to identify the normal note by actual title.
- You can link to a subheading by appending `#heading` name behind the title, for example: "Fleeting Notes#Second Brain". However, make sure to url encode the spaces (`%20`) and the "#" (`%23`) sign: `noteTitle=Fleeting%20Notes%23Second%20Brain`
- Link to a synced line by appending it's ID behind the note title separated by "^", for example: "Fleeting Notes^pxgobp". Here, too, you need to encode the special characters, spaces (`%20`) and "^" (`%5E`). For example: `noteTitle=Fleeting%20Notes%5Epxgobp`. You can look up the ID inside the raw text file or click on the drag button left of the line to copy the URL.
- **filename** optional, to identify a note by filename instead of title or date. You have to use the relative path, which means including the folders (e.g. "folder/note.txt"). If the folder or filename contains spaces and possibly other characters which are not supported in an URL, you need to URL-encode them. A space becomes "%20" for example. You get the correct URL by using the "Copy URL to Note" feature in the note menu (see menu icon top right inside the editor). Searches first regular notes and if nothing found tries to convert the filename into a date.
- **heading** optional, assign a name of a subheading inside the note so NotePlan scrolls to, and highlights this subheading. *Available from v3.7.2*
- **subWindow** optional (only Mac), values: `yes` (opens note in a subwindow) and `no`
- **splitView** optional (only Mac), values: `yes` (opens note in a split view) and `no`*. *N**ote: Available from v3.4*
*
- **reuseSplitView** optional (only Mac), values: `yes` (reuses an existing split view instead of creating a new one) and `no` (default). When set to `yes`, NotePlan will look for an existing split view editor and open the note there. If no split view exists, a new one will be created. This is useful for quickly browsing through notes without accumulating multiple split views. Must be used together with `splitView=yes`. *Note: Available from v3.20.1*
- **useExistingSubWindow** optional (only Mac), values: `yes` (looks for an existing subwindow and opens the note there, instead of opening a new one) and `no` (default). *Note: Available from v3.2*
- **highlightStart / highlightLength** optional, to set select text in the note after opening it. This also scrolls the note to that position. The start is the character index of where you want the cursor to jump and use a length > 0 if you want to select text. Use something high like 9999 if you want the cursor to jump to the last position (to the bottom, for example, if you want to create a new note and type right away). *Note: Available from v3.9*

**Example:**

`noteplan://x-callback-url/openNote?noteDate=today`

## /openView

*Available from v3.18.1
*Open a view identified by the name and folder (optional).

**Parameters:**

- **name** Name of the view (if it's not unique, combine with the folder variable)
- **folder** optional (or as alternative), but recommended to identify the right view. This is the relative path to the folder. For example "10 - Projects/Review". If the folder contains spaces and possibly other characters which are not supported in an URL, you need to URL-encode them. A space becomes "%20" for example. Right-click the folder in the sidebar and use "Copy Relative Path", then encode the special characters.

**Example:**

`noteplan://x-callback-url/openView?name=Project%20Tasks&folder=10%20-%20Projects`

## /addText

Adds text to a note identified by the title, filename (normal notes) or date (calendar notes).

**Parameters:**

- **noteDate** optional to identify the calendar note in the format YYYYMMDD like '20180122' or use 'today', 'yesterday', 'tomorrow' instead of a date.
- **noteTitle** optional to identify the normal note by actual title.
- **fileName** optional to identify a normal note by filename instead of title or date. Searches first general notes, then calendar notes for the filename (strings for dates can be also used here like for the noteDate attribute).
- **text** text to be added to the note.
- **mode** optional, values: `prepend`, `append`
- **openNote** optional, values: `yes` (opens the note, if not already selected), `no`
- **subWindow** optional (only Mac), values: `yes` (opens note in a subwindow if 'openNote' was also added as argument with 'yes' value) and `no`
- **splitView** optional (only Mac), values: `yes` (opens note in a split view) and `no`*. *N**ote: Available from v3.4**
- **useExistingSubWindow** optional (only Mac), values: `yes` (looks for an existing subwindow and opens the note there, instead of opening a new one) and `no` (default). *Note: Available from v3.2*

**Examples:**

Calendar notes:

`noteplan://x-callback-url/addText?noteDate=today&text=*%20Hello%20World&mode=append&openNote=yes`

Use `noteDate` to target calendar notes.

Regular notes:

`noteplan://x-callback-url/addText?noteTitle=Test%20Note&text=*%20Hello%20World&mode=prepend`

Use `noteTitle` to target normal notes.

## /addNote

Adds a new note with text and a title. If the title already exists, it still creates a new note and appends a number to the filename. If you want to add a calendar note, use "/addText". The calendar notes are automatically created for you and there can't exist multiple calendar notes for one date.

A new note should have either a title or a text. Empty notes are not allowed.

- **noteTitle** optional, will be prepended if it is used
- **text** optional, text will be added to the note
- **openNote** optional, values: `yes` (opens the note, if not already selected), `no`
- **folder** optional, define which folder the note should be added to. The folder will be created.
- **subWindow** optional (only Mac), values: `yes` (opens note in a subwindow) and `no`
- **splitView** optional (only Mac), values: `yes` (opens note in a split view) and `no`*. *N**ote: Available from v3.4**
- **useExistingSubWindow** optional (only Mac), values: `yes` (looks for an existing subwindow and opens the note there, instead of opening a new one) and `no` (default). *Note: Available from v3.2*
- **highlightStart / highlightLength** optional (when used together with openNote), to set select text in the note after opening it. This also scrolls the note to that position. The start is the character index of where you want the cursor to jump and use a length > 0 if you want to select text. Use something high like 9999 if you want the cursor to jump to the last position (to the bottom, for example, if you want to create a new note and type right away). *Note: Available from v3.9*

**Example:**

`noteplan://x-callback-url/addNote?noteTitle=New%20Note&openNote=yes`

## /deleteNote

- **noteTitle** title to identify a normal note
- **noteDate** date to identify a calendar note in the format YYYYMMDD like '20180122' or use 'today', 'yesterday', 'tomorrow' instead of a date.
- **fileName** optional to identify a note by filename instead of title or date. Searches first general notes, then calendar notes for the filename.

**Example:**

`noteplan://x-callback-url/deleteNote?noteTitle=New%20Note`

## /selectTag

- **name** required tag name, leave it empty to show all notes. You have to prepend it with a "#" or "@", otherwise, it will search for the given keyword.

**Example:**

`noteplan://x-callback-url/selectTag?name=#noteplan`

## /search

- **text** required to identify the search string, leave it empty to clear the search field.
- **filter** alternatively to "text", opens an existing filter instead of search.

**Example:**

`noteplan://x-callback-url/search?text=noteplan`

`noteplan://x-callback-url/search?filter=Upcoming`

## /runPlugin

- **pluginName** required to identify in which plugin NotePlan should search for the command can be alternatively pluginID. Find the plugin name in the Preferences > Plugins. You need to url-encode the name, so that spaces become "%20"
- **pluginID** as an alternative to pluginName to identify the plugin. Find the ID in the plugin.json file of the plugin stored in the Plugins folder.
- **command** required, to identify the command you want to run. Without a leading "/". For example command "/nc" becomes "nc"
- **arg0, arg1, arg2**, ... (optional) to pass arguments to the plugin if supported (*available in v3.5*).

Example:

`noteplan://x-callback-url/runPlugin?pluginName=🔢%20Note%20Statistics&command=nc`

## /installPlugin

- pluginID to identify the plugin. Find the ID in the plugin.json file of the plugin stored in the Plugins folder.

Example:

`noteplan://x-callback-url/installPlugin?pluginID=dwertheimer.Favorites`

*Note: Available from v3.15*

## /toggleSidebar

Toggle, show, or hide the sidebar. Note: Available in v3.19.2

**Parameters:**

- **forceCollapse** optional, values: `yes` (forces the sidebar to hide/collapse) and `no` (default)
- **forceOpen** optional, values: `yes` (forces the sidebar to show/expand) and `no` (default)
- **animated** optional (only Mac), values: `yes` (default, animates the sidebar toggle) and `no` (instantly shows/hides the sidebar without animation)

**Note:** If both `forceCollapse` and `forceOpen` are set to `yes`, `forceOpen` takes precedence on macOS.

**Examples:**

Toggle the sidebar (show if hidden, hide if shown):
`noteplan://x-callback-url/toggleSidebar`

Force show/open the sidebar:
`noteplan://x-callback-url/toggleSidebar?forceOpen=yes`

Force hide/collapse the sidebar:
`noteplan://x-callback-url/toggleSidebar?forceCollapse=yes`

Force hide the sidebar without animation (Mac only):
`noteplan://x-callback-url/toggleSidebar?forceCollapse=yes&animated=no`

## /noteInfo (x-success)

This only works with x-success and is to get the absolute filepath and name to the currently opened note.

Result parameter:

- **path** absolute file path to the opened note file.
- **name** name of the opened note.

**Example:**

`noteplan://x-callback-url/noteInfo/?x-success=sourceapp://x-callback-url`

**Result (NotePlan will call):**

`sourceapp://x-callback-url?path=[html encoded file path]&name=[note name]`

Where the parameters in square brackets are replaced by the actual path and name, both html encoded. Replace "sourceapp://x-callback-url" with the x-callback-url link of the app you want to call or the source app from where you call it.

## x-success

*Available from NotePlan v3.5*

Every URL supports the x-success parameter. Using this you can return to the original app after the x-callback-url has been processed in NotePlan.

**Example:**

`noteplan://x-callback-url/addText?noteDate=today&text=Hello&x-success=sourceapp://x-callback-url`

##

This example will append the text "Hello" at the end of today's note and return to the calling app (or any other app). Edit the `sourceapp://x-callback-url` part to match it with the API of the next app you want to call.

If you are using Shortcuts, you don't need to add this parameter. Instead, just use the action `open Text with x-callback` and Shortcuts will automatically add the right x-callback parameter. If you don't want to return to Shortcuts, use `open URL`.

## Testing

Run the X-Callback-Url like this in the Terminal to test:

`open "noteplan://x-callback-url/addText?noteDate=20180204&text=Hello%20World"`

Or enter the URL into Safari or another browser.

*Last updated: 2026-01-27*
