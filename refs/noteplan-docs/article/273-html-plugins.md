# Creating HTML View Plugins (using AI)

This guide helps you create HTML view plugins for NotePlan using AI. There are two approaches depending on your setup.

In this article:

- [Option A: Using the NotePlan MCP Server (Recommended)](#option-a)
- [Option B: Using Any AI Tool (Manual Setup)](#option-b)
- [Step 1: Download the Plugin Template](#step-1)
- [Step 2: Install the Plugin](#step-2)
- [Step 3: Ask the AI to Build Your Plugin](#step-3)
- [Step 4: Launch Your Plugin](#step-4)
- [Plugin File Structure](#file-structure)
- [New Format: plugin.type: "html"](#new-format)
- [How the Files Connect](#files-connect)
- [Backward Compatibility: The script.js Shim](#backward-compat)
- [Calling NotePlan APIs from HTML Views](#apis-from-html)
- [Lifecycle Events (v3.21+)](#lifecycle-events)
- [Real-World Example: Stock Ticker Plugin](#example)

## Option A: Using the NotePlan MCP Server (Recommended)

The NotePlan MCP server is the easiest way to create plugins. It connects AI assistants directly to NotePlan, so they can create, update, reload, and screenshot plugins automatically.

Compatible AI tools:

- [Claude Code](https://claude.com/product/claude-code) — Terminal-based AI coding assistant
- [Claude Desktop](https://claude.com/download) — Desktop AI assistant with MCP support
- [Codex](https://openai.com/index/codex/) — OpenAI’s coding agent
- Any AI tool that supports the [Model Context Protocol](https://modelcontextprotocol.io/)

When you use the MCP server, the AI handles everything:

- Creates the plugin folder with all required files
- Writes index.html, styles.css, app.js as separate files
- Generates plugin.json with the new plugin.type: "html" format
- Generates the script.js backward-compatibility shim automatically
- Reloads plugins and launches the view so you can see it instantly

Just describe what you want and the AI takes care of the rest. To iterate, describe changes and the AI updates the files and reloads.

For setup instructions, see [How to Install the NotePlan MCP Server](277-how-to-install-the-noteplan-mcp-server.md).

## Option B: Using Any AI Tool (Manual Setup)

If you’re not using the MCP server, you can use any AI tool (Cursor, ChatGPT, etc.) to generate the plugin files. You’ll need to set up the file structure yourself and tell the AI what to create.

### Prerequisites

- NotePlan 3.20 or later (3.21+ for the new HTML plugin format)
- Any AI coding assistant (Cursor, ChatGPT, Claude, etc.)

No JavaScript experience required — the AI will generate the code for you.

### Step 1: Download the Plugin Template

[Download](https://media.noteplan.co/files/np.myplugin.zip) the plugin template ZIP file and extract it.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/69569b0eb403632922ebefb3/file-lqGC51Ddly.png)

The template includes:

- getting-started.md — basic setup instructions (you or your AI can read this)
- plugin.json — plugin metadata and configuration
- script.js — example plugin code with HTML view examples
- API documentation files — reference for the AI

### Step 2: Install the Plugin

Move the extracted plugin folder into NotePlan’s Plugins directory and rename it.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/69569b9e194028262d51581d/file-g27seVr2Mx.png)

- Open NotePlan
- Go to Settings → AI & Plugins
- Click Open Plugins Folder
- Copy np.myplugin into the Plugins folder
- Rename it, for example to em.LinearCalendar (using your initials is recommended)

### Step 3: Ask the AI to Build Your Plugin

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/69569cf6194028262d51581f/file-ZwgpKDu2TQ.gif)

Open the plugin folder in your AI tool and ask it to create an HTML view plugin. You can use this prompt as a starting point:

-
-
-

-
-
-

> I want to create a NotePlan plugin using the provided template and API files.
> Plugin name: YOUR_PLUGIN_NAME
> Plugin description: SHORT DESCRIPTION OF WHAT THE PLUGIN DOES
> Please set this up as an HTML View plugin that:
> Opens a visible HTML view in the main NotePlan editor area
> Adds one clear command to launch this HTML view
> Updates plugin.json and script.js with a suitable name, description, command structure
> Context:
> Read the file getting-started.md to learn about NotePlan plugins
> Find in this project a template plugin.json and script.js file to modify
> Read the included API documentation files like HTMLView.md, NotePlan.md, etc.

### Step 4: Launch Your Plugin

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/69569ea8b403632922ebefb7/file-obQVrWo6sf.gif)

In NotePlan, press CMD + J, then type /. Type your plugin name or command name to find it.

Once launched, your plugin appears in the sidebar and opens in the main editor area. To iterate, describe changes to the AI, let it update the code, then click the Reload button next to the plugin title.

## Plugin File Structure

Starting with NotePlan 3.21, plugins can use a cleaner format where HTML, CSS, and JavaScript live in separate files.

### New Format: plugin.type: "html" (NotePlan 3.21+)

NotePlan loads index.html directly — no JavaScript wrapper needed. Your plugin folder looks like this:

```
np.myPlugin/
  plugin.json       ← plugin metadata with "plugin.type": "html"
  index.html        ← your HTML file
  styles.css        ← CSS, linked via <link> in index.html
  app.js            ← JavaScript, linked via <script> in index.html
  script.js         ← backward-compat shim (see below)
```

### How the Files Connect

plugin.json is the entry point. It tells NotePlan how to load the plugin:

```
{
  "plugin.id": "np.myPlugin",
  "plugin.name": "My Plugin",
  "plugin.description": "Description of the plugin",
  "plugin.author": "Your Name",
  "plugin.version": "1.0.0",
  "plugin.icon": "star",
  "plugin.iconColor": "blue-500",

  "plugin.type": "html",
  "plugin.html": "index.html",
  "plugin.sidebarView": true,
  "plugin.sidebarTitle": "My Plugin",

  "plugin.script": "script.js",
  "plugin.commands": [
    {
      "name": "My Plugin",
      "description": "Opens the plugin view",
      "jsFunction": "myPlugin",
      "sidebarView": {
        "title": "My Plugin",
        "icon": "star",
        "iconColor": "blue-500"
      }
    }
  ]
}
```

Key fields:

- "plugin.type": "html" — tells NotePlan 3.21+ to load the HTML file directly
- "plugin.html": "index.html" — which HTML file to load
- "plugin.sidebarView": true — show this plugin in the sidebar
- "plugin.sidebarTitle" — (optional) sidebar display title, falls back to plugin.name if not set
- "plugin.script" and "plugin.commands" — needed for backward compatibility with NotePlan < 3.21

index.html is your plugin’s UI. It’s a standard HTML file that references your CSS and JS:

```
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link rel="stylesheet" href="styles.css">
</head>
<body>
  <h1>Hello from my plugin!</h1>
  <div id="content"></div>
  <script src="app.js"></script>
</body>
</html>
```

styles.css and app.js are standard CSS and JavaScript files. Nothing special — they’re loaded by the browser just like any web page.

### Backward Compatibility: The script.js Shim

If you want your plugin to also work on NotePlan versions older than 3.21, you need a script.js file. This is a small wrapper that embeds your index.html content and displays it using the legacy HTMLView.showInMainWindow() API.

The shim simply embeds the index.html — since the legacy path writes a temp file in the same plugin folder, <link href="styles.css"> and <script src="app.js"> still resolve correctly. No need to inline the CSS or JS.

Example script.js:

```
// Backward-compat shim for NotePlan < 3.21
// If using the new plugin.type: "html" format, this file is only
// loaded by older NotePlan versions. Edit index.html instead.
globalThis.myPlugin = async function() {
  const html = `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <link rel="stylesheet" href="styles.css">
</head>
<body>
  <h1>Hello from my plugin!</h1>
  <div id="content"></div>
  <script src="app.js"></script>
</body>
</html>`;
  await HTMLView.showInMainWindow(html, "My Plugin", {
    "id": "main:np.myPlugin:My Plugin",
    "icon": "star"
  });
};
```

The jsFunction in plugin.json must match the function name in script.js (in this example, "myPlugin" matches globalThis.myPlugin).

If you’re using the MCP server, script.js is generated automatically. If you’re writing the plugin manually or with an external AI, tell the AI to create this file as well.

## Calling NotePlan APIs from HTML Views

HTML views can call a subset of NotePlan’s JavaScript APIs directly from the page — no `jsBridge` round-trip needed. This is how plugins read events, query notes, access the clipboard, and more without having to round-trip through `script.js`.

**Currently available directly from HTML:**

- `Calendar` — events and reminders
- `DataStore` — read and search notes
- `Editor` — current editor content
- `Note` — mutate notes returned by DataStore/Editor (notes are auto-proxied)
- `NotePlan` — environment info, `selectedSidebarFolder`, `openURL`, and more
- `Clipboard` — read/write the system clipboard
- `fetch()` — overridden to be CORS-free, so you can call any external API directly

**Not yet available from HTML views** (still require a `jsBridge` round-trip to `script.js`): `CommandBar`, `HTMLView`, and any plugin-defined commands.

**Three rules for bridge calls:**

1. **Every call is async.** Every bridge call returns a Promise — always `await` it. Without `await`, the next line can race ahead of the call’s effect.
```
// ✅ correct
const events = await Calendar.eventsToday()

// ❌ wrong — `events` is a Promise, not an array
const events = Calendar.eventsToday()
```

2. **Method-call form only — no property writes.** JavaScript assignment (`=`) does not cross the bridge. Use the matching setter method instead.
```
// ✅ correct
await note.setContent("new body")
await Clipboard.setString("hello")

// ❌ wrong — assigns to the local JS proxy and is silently lost
note.content = "new body"
Clipboard.string = "hello"
```

3. **Property reads work via `await`.** Simply awaiting a property returns its current value.
```
const env = await NotePlan.environment
const folder = await NotePlan.selectedSidebarFolder
const types = await Clipboard.types
```

**Creating events and reminders.** The `CalendarItem.create(...)` factory is not exposed to HTML views, but you don’t need it: `Calendar.add(...)` accepts a plain object literal with the same fields and auto-converts it. In the plugin JS core you can use either form; in HTML views the dict form is the only option.

```
// Create an event
await Calendar.add({
  title: "Team sync",
  date: new Date("2026-05-01T10:00"),
  endDate: new Date("2026-05-01T10:30"),
  type: "event",
  isAllDay: false,
  calendar: "Work",
  notes: "Agenda: …"
})

// Create a reminder
await Calendar.add({
  title: "Call the plumber",
  date: new Date("2026-05-02T09:00"),
  type: "reminder",
  isCompleted: false,
  calendar: "Reminders"
})
```

**Updating a paragraph.** `note.updateParagraph(dict)`, `removeParagraph(dict)`, and `insertParagraphAfterParagraph(content, dict, type)` accept the paragraph snapshots you receive from `note.paragraphs`. Spread the received dict when editing — don’t rebuild it from scratch, or the bridge can’t locate the target line (it matches on hidden `originalContent`/`originalType`/`originalIndents`/`originalHeadingLevel` fields that the serializer includes automatically).

```
const paras = note.paragraphs
const edited = { ...paras[3], content: "rewritten" }  // ✅ keeps originals
await note.updateParagraph(edited)

// ❌ missing originals — update silently no-ops
await note.updateParagraph({ lineIndex: 3, content: "rewritten" })
```

Why these rules? The HTML view runs in a separate WebKit process and reaches NotePlan through asynchronous `postMessage`. Unlike the plugin’s `script.js` (which runs in the same JavaScriptCore context as NotePlan and can call APIs synchronously), HTML views must wait for a round-trip every time — so assignments can’t be reliably awaited, and every call returns a Promise.

Full per-API documentation (methods, parameters, return types, and the setter-method equivalents for property assignment) lives in the [JavaScript Plugin API reference](70-javascript-plugin-api.md).

## Lifecycle Events (v3.21+)

When a plugin view is shown or hidden — for example, when the user switches between a note and your plugin — NotePlan dispatches DOM events to your HTML view. This lets you refresh data or pause timers without a full reload.

Available events:

- onViewDidAppear — fired when your plugin view becomes visible again
- onViewWillDisappear — fired when your plugin view is about to be hidden

These fire in all display modes: main view, split view, and standalone windows.

Usage:

```
window.addEventListener('onViewDidAppear', () => {
  // Refresh data, resume timers, etc.
  refreshData();
});

window.addEventListener('onViewWillDisappear', () => {
  // Pause timers, save state, etc.
  pauseUpdates();
});
```

These work alongside the existing notePlanBridgeReady event:

- notePlanBridgeReady fires once when the NotePlan API bridge is first initialized
- onViewDidAppear fires each time the view becomes visible (not on first load)
- onViewWillDisappear fires each time the view is about to be hidden

## Real-World Example: Stock Ticker Plugin

The [Stock Ticker plugin](https://github.com/EduardMe/noteplan-stock-ticker-plugin) is a complete example of the new HTML plugin format with separated files:

```
np.stockTicker/
  plugin.json       ← plugin.type: "html" + legacy shim fields
  index.html        ← HTML shell (~900B) with <link> and <script> tags
  styles.css        ← all CSS (7.6KB)
  app.js            ← all JavaScript (25KB)
  script.js         ← backward-compat shim (1.2KB)
```

Browse the source on GitHub to see exactly how the files are structured and connected.

*Last updated: 2026-04-18*
