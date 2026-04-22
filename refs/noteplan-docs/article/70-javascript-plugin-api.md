# Javascript Plugin API

NotePlan provides various global Javascript objects to give you access to your data and to prompt user input. Here is a list of available global objects:

```
// Interfaces
NotePlan	// Global environment information and other functions
Editor 		// Access to the markdown editor
DataStore 	// Query, read and modify notes
CommandBar 	// Prompt user input
Calendar 	// Create events and reminders
Clipboard	// Access and set the clipboard
HTMLView	// Open a window or sheet with html code that can interact with NotePlan

// Helper objects (usually as return values)
ParagraphObject	// Line of text, like a task, bullet...
NoteObject	// Represents a note file
RangeObject	// Range of text like a selection
CalendarItem	// Event or reminder

PluginObject	// Represents an available or installed plugin (documented in DataStore)
PluginCommandObject // Represents a command of a plugin (documented in DataStore)

// Global functions
fetch()
```

Learn more about:

- [NotePlan](#noteplan)
- [Editor](#editor)
- [DataStore](#datastore)
- [CommandBar](#commandbar)
- [Calendar](#calendar)
- [ParagraphObject](#paragraphobject)
- [NoteObject](#noteobject)
- [RangeObject](#rangeobject)
- [CalendarItem](#calendaritem)
- [Clipboard](#clipboard)
- [HTMLView](#htmlview)
- [Error Handling](#errorhandling)
- [fetch()](#fetch)

---

## [NotePlan](#noteplan)

Access environment information like region code, language code, timezone.

[Jump back](#noteplan)

## [Editor](#editor)

This refers to the markdown editor and the currently opened note. You can access the text directly from here, change the selection and even highlight parts.

However, be careful about character positions, because NotePlan hides Markdown characters and replaces whole parts of text such as the URL in Markdown links or folded text with a single symbol. This can make calculating character positions and changing the text a bit tricky. Prefer working with the paragraph objects instead to modify text directly.

Here are the available functions you can call with the Editor object:

[Jump back](#editor)

## [DataStore](#datastore)

With `DataStore` you can query, create and move notes which are cached by NotePlan. It allows you to query a set of user preferences, too.

[Jump back](#datastore)

## [CommandBar](#commandbar)

Use `CommandBar` to get user input. Either by asking the user to type in a free-form string, like a note title, or by giving him a list of choices. This list can be "fuzzy-search" filtered by the user. So, it's fine to show a long list of options, like all folders or notes or tasks in a note.

[Jump back](#commandbar)

## [Calendar](#calendar)

Use `Calendar` to create events, reminders, and to parse dates, like "tomorrow at 8am to 10am", "today", or "1st May". See also [CalendarItem](#calendaritem) if you want to create an event or reminder.

[Jump back](#calendar)

## [ParagraphObject](#paragraphobject)

You can get paragraphs from [Editor](#editor) or [NoteObject](#noteobject). They represent blocks or lines of text (and are delimited by linebreaks = `\n`). A task for example is a paragraph, a list item (bullet), heading, etc. (see `.type`).

[Jump back](#paragraphobject)

## [NoteObject](#noteobject)

Notes can be queried by [DataStore](#datastore). You can change the complete text of the note, which will be saved to file or query, add, remove, or modify particular paragraphs (a paragraph is a task for example). See more paragraph editing examples under [Editor](#editor). `NoteObject` and `Editor` both inherit the same paragraph functions.

[Jump back](#noteobject)

## [RangeObject](#rangeobject)

Ranges are used when you deal with selections or need to know where a paragraph is in the complete text.

[Jump back](#rangeobject)

## [CalendarItem](#calendaritem)

The `CalendarItem` is used in combination with [Calendar](#calendar) to create events or reminders.

[Jump back](#calendaritem)

## [HTMLView](#htmlview)

Open a new window or sheet with the `HTMLView` that loads up the given HTML code which can interact with NotePlan's data.

[Jump back](#htmlview)

## [Clipboard](#clipboard)

Access and set the data inside the current clipboard.

[Jump back](#clipboard)

## [Error Handling](#errorhandling)

If you are using `await` in your code, wrap it in a `try-catch` block to get the error messages. Otherwise, the plugin will silently fail without an error message, because it runs asynchronous:

```
try { 
   console.log("Hello World") 
} catch (error) { 
   console.log("Plugin code error: \n"+JSON.stringify(error)) 
}
```

## [fetch()](#fetch)

Access resources from the web with this function. It supports the following parameters as an object in the second argument:

- **timeout**: Set an alternative timeout as milliseconds. The default is 60s (available in NotePlan v3.5).
- **method**: Change the HTTP method like "POST".** **
- **headers**: Set HTTP header fields like for authentication.
- **body**: Set the HTTP body like attaching data to your request.

```
function test() {
    fetch("https://noteplan.co", { timeout: 1000 })
	.then(n => console.log(n))
  	.catch(error => console.log("error: " + error))
}
```

Note: Fetch only works with websites that have an SSL certificate installed (i.e.: start with "https://...").

### Fetch Error Handling

To handle errors when calling fetch (and you should, because it is very likely that the internet connection is unstable or the website is not accessible), you need to use the Promises functions `then` and `catch` like in the example above. Don't use `const re = await fetch()`, because you can't catch errors this way.

The error object will be a string explaining the problem. For example, a timeout error will return: "The request timed out."

---

**Next up:** [Plugin Hooks →](137-plugin-hooks.md)

**Jump to:**

- [Plugins](65-commandbar-plugins.md)
- [Create Plugins](67-create-command-bar-plugins.md)
- [Javascript Plugin API](70-javascript-plugin-api.md) *(you're here)*
- [Plugin Hooks](137-plugin-hooks.md)
- [Releasing a plugin](80-releasing-a-plugin.md)

*Last updated: 2024-01-17*
