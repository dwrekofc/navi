# AI Prompts in Templates

Use AI prompts in your templates to add content to your notes. You can give it access to specific notes (regular or calendar) to compile content from your existing notes.

- [Prompt Function](#prompt-function)
- [Send Regular Notes](#send-regular-notes)
- [Send Calendar Notes](#send-calendar-notes)
- [Send a Series of Calendar Notes](#send-series-of-calendar-notes)
- [Send a Folder of Regular Notes](#send-folder-of-regular-notes)
- [Define a Model](#define-a-model)
- [Examples](#examples)

## Prompt Function

The NotePlan.ai function allows you to use AI within your templates. It's an async function, so you must use it with `await`. It sends a prompt to OpenAI and returns the result as text.

```
/*
* @param { String }
* @param { String[] }
* @param { Boolean }
* @param { String } // from v3.16.3
* @return { Promise<String> }
*/
NotePlan.ai(prompt, filenames, useStrictFilenames, model)

// Example (inside a template):
<%- await NotePlan.ai("Summarize this note", ["today"]) %>
```

### Send Regular Notes

Optionally send the content of your notes by specifying them in the second argument 'filenames', as a list. For example ["note1.md", "folder/note2.md"]. This needs to be the exact path to the note. Your note extension might differ, the default is .txt, if you haven't changed it.

```
// Access regular notes by filename
<%- await NotePlan.ai("Summarize projects", ["Projects/Project A.md", "Projects/Project B.md"]) %>
```

### Send Calendar Notes

For calendar notes, you can use `YYYYMMDD.md`, like `20241101.md`, or `2024-W10.md` for weeks, etc. Natural language input is also supported like "this week", "today", "tomorrow", "this month", "next year", etc.

Use the third argument (adding 'true') if you don't want to use natural language processing like "this week" (in case you have a note that's called this).

```
// Using natural language for calendar notes
<%- await NotePlan.ai("Summarize my week", ["this week"]) %>

// Using exact calendar note format
<%- await NotePlan.ai("List all events", ["20241225.md"]) %>

// Disabling natural language processing
<%- await NotePlan.ai("Extract content from note named 'this week'", ["this week"], true) %>
```

### Send a Series of Calendar Notes

If you need to send a relative list of calendar notes, every note of the "last 7 days", you can use exactly this as the filename. The structure is as followed:

1. use "next" or "last",
2. define a number, like "7",
3. define one of the timeframes: "days", "weeks", "months", "quarters", "years".

The timeframe also defines what kind of note is being accessed. Use "weeks" if you want to send weekly notes, "days" for daily notes etc.

```
// Access the last 7 daily notes
<%- await NotePlan.ai("Summarize my recent activities", ["last 7 days"]) %>

// Access the next 2 weeks
<%- await NotePlan.ai("What's coming up?", ["next 2 weeks"]) %>
```

### Send a Folder of Regular Notes

You can also define a folder to send all the notes inside this folder. Use the path of the folder prefixed with "/", like "/Projects/Work".

```
// Send all notes from a folder
<%- await NotePlan.ai("Summarize all my work projects", ["/Projects/Work"]) %>
```

### Define a Model

If you are using your own [Open AI API key](213-where-do-i-enter-my-openai-key.md), you can define a model, for example "o1", or "o3-mini". By default NotePlan uses GPT-4o.

```
// Use a specific OpenAI model
<%- await NotePlan.ai("Generate creative ideas", ["Project Ideas.md"], false, "o1") %>
```

## Examples

Example 1 (inside a template):

```
**Random Annual Goal:**
<%- await NotePlan.ai("Return a bullet. Not just the first one, make it random", ["this year"]) %>
```

This will pick a random bullet point from your yearly note (for the current year), and add it to the current note. The first argument is the prompt and the second is a list of notes to reference, specifically this year's calendar note. Example 2:
```
**Think:**
<%- await NotePlan.ai("Return 3 stoic inspirational quotes") %>
```

This will just generate 3 stoic quotes, without referencing any of your notes.  ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/675763cfeeec4b48a76cdc88/file-yymcHT4ndC.gif)

*Last updated: 2025-05-18*
