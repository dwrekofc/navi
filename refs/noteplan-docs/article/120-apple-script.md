# Apple Script

NotePlan supports AppleScript commands on macOS for automating common tasks. You can run these from Script Editor, Terminal (osascript), or any automation tool that supports AppleScript.

You can also run any command directly from Terminal using `osascript -e`:

```
osascript -e 'tell application "NotePlan" to listNotes'
```

**Tip for AI agents:** This article can be passed to AI agents (such as ChatGPT, Claude, or Cursor) so they can control NotePlan via AppleScript. For a more integrated experience, you can also use the [NotePlan MCP server](277-how-to-install-the-noteplan-mcp-server.md) with AI clients like [Claude Desktop, Claude Code, or Codex](article/279-install-the-noteplan-mcp-server-in-claude-desktop-claude-code-and-codex.md).

## Getting Note Info

Get the URL of the currently opened note:

```
tell application "NotePlan"
 set x to selectedNoteUrl
 return x
end tell
```

Get the title of the currently opened note:

```
tell application "NotePlan"
 set x to selectedNoteTitle
 return x
end tell
```

Get the filename of the currently opened note *(available in v3.20.2+)*:

```
tell application "NotePlan"
 set x to selectedNoteFilename
 return x
end tell
```

Returns the relative path of the note, such as `Folder/My Note.md`.

Get the app version and build number (JSON) *(available in v3.20.2+)*:

```
tell application "NotePlan" to getVersion
-- Returns: {"version":"3.20.2","build":1234}
```

## Creating & Opening Notes

Create a new note:

```
tell application "NotePlan"
 addNote with title "Meeting Notes" and content "Agenda items here" and folder "Work"
end tell
```

All parameters (`with title`, `and content`, `and folder`) are optional.

Open a note by title or path *(available in v3.20.2+)*:

```
tell application "NotePlan"
 showNote titled "Meeting Notes"
 showNote at path "Work/Meeting Notes.txt"
end tell
```

Optional flags: `in new window`, `in split view`, `in background`.

```
tell application "NotePlan"
 showNote titled "Meeting Notes" in new window true
 showNote titled "Meeting Notes" in split view true in background true
end tell
```

Open today's daily note *(available in v3.20.2+)*:

```
tell application "NotePlan" to openToday
```

## Reading & Writing Notes

All commands in this section are *available in v3.20.2+*.

Read the content of a note:

```
tell application "NotePlan"
 readNote titled "My Note"
 readNote at path "Work/My Note.txt"
 readNote for date "2026-02-17"
end tell
```

You can target a note by title, filename/path, or date (for calendar notes). Returns the full text content.

Replace the entire content of a note:

```
tell application "NotePlan"
 writeNote with content "New content here" titled "My Note"
 writeNote with content "New content here" at path "Work/My Note.txt"
 writeNote with content "Updated daily plan" for date "today"
end tell
```

Insert content into a note at a specific position:

```
-- Append to the end (default)
tell application "NotePlan"
 insertContent with content "New paragraph" in note titled "My Note"
end tell

-- Insert at the start (after frontmatter)
tell application "NotePlan"
 insertContent with content "Top note" in note titled "My Note" at position "start"
end tell

-- Insert after a specific heading
tell application "NotePlan"
 insertContent with content "- Action item" in note titled "My Note" at position "after-heading" under heading "Tasks"
end tell

-- Insert at the end of a section
tell application "NotePlan"
 insertContent with content "Last item in section" at path "Work/Project.txt" at position "end-of-section" under heading "Tasks"
end tell
```

Supported positions: `start`, `end` (default), `after-heading`, `end-of-section`.

Delete a note (move to trash):

```
tell application "NotePlan"
 deleteNote titled "Old Note"
 deleteNote at path "Work/Old Note.txt"
 deleteNote for date "2026-01-01"
end tell
```

For calendar notes, this clears the content rather than deleting the file.

## Searching & Listing Notes

Search notes in the UI *(available in v3.20.2+)*:

```
tell application "NotePlan"
 searchNotes for "project plan"
 searchNotes for "project plan" in background true
end tell
```

This opens NotePlan's search view with the results.

Search notes and get results as JSON *(available in v3.20.2+)*:

```
-- Search in both title and content
tell application "NotePlan" to findNotes for "meeting agenda"

-- Search only in titles
tell application "NotePlan" to findNotes for "project plan" in field "title"

-- Search within a specific folder
tell application "NotePlan" to findNotes for "TODO" in folder "Work"
```

Returns a JSON array. Each result includes `title`, `filename`, `folder`, and `matchedIn` (where the match was found). The `in field` parameter accepts `title`, `content`, or `both` (default).

List notes as JSON *(available in v3.20.2+)*:

```
-- List all notes
tell application "NotePlan" to listNotes

-- List notes in a specific folder
tell application "NotePlan" to listNotes in folder "Projects"

-- List all folders
tell application "NotePlan" to listNotes of type "folders"

-- List calendar notes
tell application "NotePlan" to listNotes of type "calendar"
```

Returns a JSON array. For notes: `title`, `filename`, `folder`, `modifiedAt`. For folders: `title`, `filename`, `isFolder`. For calendar notes: `filename`, `date`, `timeframe`.

## Templates

All commands in this section are *available in v3.20.2+*.

Render a saved template by title:

```
tell application "NotePlan"
 renderTemplate with title "Daily Template"
end tell
```

Render arbitrary template code:

```
tell application "NotePlan"
 renderTemplate with content "Today is <%- date.now('YYYY-MM-DD') %>"
end tell
```

Returns JSON with `success` and `rendered` fields.

## Plugins

All commands in this section are *available in v3.20.2+*.

Run a plugin command:

```
tell application "NotePlan"
 executePlugin with id "np.Templating" with command "templateRunner"
end tell
```

You can pass arguments as a JSON string:

```
tell application "NotePlan"
 executePlugin with id "my.plugin" with command "doSomething" with arguments "{\"key\":\"value\"}"
end tell
```

Reload all plugins:

```
tell application "NotePlan" to reloadPlugins
```

List installed plugins (JSON):

```
tell application "NotePlan" to listInstalledPlugins
```

List available plugins from the online repository (JSON):

```
tell application "NotePlan" to listAvailablePlugins
tell application "NotePlan" to listAvailablePlugins include beta true
```

Install or update a plugin:

```
tell application "NotePlan" to installPlugin with id "np.Templating"
```

## Views & UI

All commands in this section are *available in v3.20.2+*.

Open a named view:

```
tell application "NotePlan" to openView named "Review"
```

Toggle the sidebar:

```
tell application "NotePlan" to toggleSidebar
```

Set the theme:

```
tell application "NotePlan" to setTheme to "Alabaster" for mode "light"
tell application "NotePlan" to setTheme to "Nord" for mode "dark"
```

Mode can be `light`, `dark`, or `auto`.

## Plugin Windows

All commands in this section are *available in v3.20.2+*.

List open plugin windows (JSON):

```
tell application "NotePlan" to listPluginWindows
```

Close a plugin window:

```
tell application "NotePlan" to closePluginWindow with id "window:my.plugin:Title"
tell application "NotePlan" to closePluginWindow titled "My Plugin Window"
-- Close all plugin windows:
tell application "NotePlan" to closePluginWindow
```

Screenshot a plugin window (saves as PNG):

```
tell application "NotePlan" to screenshotPlugin with id "my.plugin"
```

## Backups

Create a full backup of all data *(available in v3.20.2+)*:

```
tell application "NotePlan" to createBackup
```

Returns JSON with `success` and `timedOut` fields. This may take a while for large vaults.

## Embeddings

Generate an embedding vector for text (requires OpenAI API key) *(available in v3.20.2+)*:

```
tell application "NotePlan" to embedText for "Some text to embed"
```

Returns JSON with the embedding array, model, and dimension count.

## Calendar Events

All commands in this section are *available in v3.20.2+*.

Dates can be in any of these formats: `2026-02-23T14:00:00Z` (ISO 8601 with timezone), `2026-02-23T14:00:00` (local time), `2026-02-23 14:00` (local time), or `2026-02-23` (date only, midnight).

List all calendars (JSON):

```
tell application "NotePlan" to listCalendars
```

Returns a JSON array with `name`, `id`, `color`, `source`, and `isWritable` for each calendar.

List events in a date range (JSON):

```
-- Events for a single day
tell application "NotePlan" to listEvents from date "2026-02-23" to date "2026-02-24"

-- Events for the next 7 days
tell application "NotePlan" to listEvents from date "2026-02-23" to date "2026-03-02"

-- Filter by calendar name
tell application "NotePlan" to listEvents from date "2026-02-23" to date "2026-02-24" in calendar "Work"
```

Returns a JSON array of events with `id`, `title`, `startDate`, `endDate`, `allDay`, `calendar`, `location`, and `notes`.

Create an event:

```
-- Timed event
tell application "NotePlan" to createEvent with title "Team Standup" from date "2026-02-23 10:00" to date "2026-02-23 10:30"

-- All-day event with optional parameters
tell application "NotePlan" to createEvent with title "Offsite" from date "2026-02-24" to date "2026-02-24" in calendar "Work" at location "Conference Room" with notes "Bring laptop" all day true
```

Returns JSON with `success` and `id`.

Update an event:

```
tell application "NotePlan" to updateEvent with id "EVENT-ID-HERE" with updates "{\"title\":\"New Title\",\"location\":\"Room 2\"}"
```

The `with updates` parameter is a JSON string. Supported keys: `title`, `startDate`, `endDate`, `location`, `notes`.

Delete an event:

```
tell application "NotePlan" to deleteEvent with id "EVENT-ID-HERE"
```

Returns JSON with `success`.

## Reminders

All commands in this section are *available in v3.20.2+*.

List all reminder lists (JSON):

```
tell application "NotePlan" to listReminderLists
```

Returns a JSON array with `name` and `id` for each list.

List reminders (JSON):

```
-- All incomplete reminders
tell application "NotePlan" to listReminders

-- Filter by list
tell application "NotePlan" to listReminders in list "Shopping"

-- Include completed reminders
tell application "NotePlan" to listReminders include completed true
```

Returns a JSON array of reminders with `id`, `title`, `completed`, `list`, `dueDate`, `notes`, and `priority`.

Create a reminder:

```
-- Simple reminder
tell application "NotePlan" to createReminder with title "Buy groceries"

-- With all options
tell application "NotePlan" to createReminder with title "Submit report" in list "Work" due date "2026-02-25 17:00" with notes "Q1 financials" with priority 1
```

Priority values: 0 (none), 1 (high), 5 (medium), 9 (low). Returns JSON with `success` and `id`.

Complete a reminder:

```
tell application "NotePlan" to completeReminder with id "REMINDER-ID-HERE"
```

Update a reminder:

```
tell application "NotePlan" to updateReminder with id "REMINDER-ID-HERE" with updates "{\"title\":\"Updated title\",\"dueDate\":\"2026-03-01 09:00\",\"priority\":5}"
```

Supported update keys: `title`, `dueDate`, `notes`, `priority`, `list`.

Delete a reminder:

```
tell application "NotePlan" to deleteReminder with id "REMINDER-ID-HERE"
```

Returns JSON with `success`.

*Last updated: 2026-03-04*
