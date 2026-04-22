# Run JavaScript code in your templates

Embed simple JavaScript code into your templates to automate your note-taking, such as "Moving tasks from yesterday into today's note and marking them as scheduled" (see examples below), or adding more dynamic information to your notes like the date, weather, etc.

In order to embed code, Templates use a special tag to identify executable code. Learn the basics in this article and find the [details here](260-template-tags.md).

You can execute JavaScript code in your templates and the results will be printed into your note. You also have access to [NotePlans full JavaScript API](70-javascript-plugin-api.md).

For example:

```
- <%- DataStore.projectNotes.length %>
```

Will print the total amount of your regular notes you have at the moment:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6298e7db5732000792520f57/file-FpxECLu3Qb.png)

##

## Writing code inside template tags

Every [tag](260-template-tags.md) ( `<%- code.returning.something() %>`) can execute one line of code and return something.

If you want to set a variable that you want to use in subsequent tags/code or execute code without printing anything, use the special tags ( `<% code.setting.or.executing() -%>`), for example:

```
<% const yesterday = new Date(new Date().setDate(new Date().getDate()-1)) -%>
```

Note: The dash "-" is placed at the closing tag, not at the opening one. This will not print anything into your note and just set the variable.

Now we can use the variable in the next tag, for example, to retrieve yesterday's note:

```
<%- DataStore.calendarNoteByDate(yesterday).filename %>
```

Or execute some code, such as opening yesterday's note in a split view:

```
<% const yesterday = new Date(new Date().setDate(new Date().getDate()-1)) -%>
<% Editor.openNoteByDate(yesterday, false, 0, 0, true) -%>
```

If you are using a lot of code in your templates, you can simplify things by using an [embedded "templatejs" code block](254-embedded-template-code-templatejs.md).

## Examples

The following examples should get you a headstart so you can copy and paste and modify the code as you need it.

##

### Copy yesterday's tasks

The following code will lookup the daily note from yesterday and take all open tasks and print them into your note if you are using it inside a template:

```
---
title: Copy yesterday's tasks
type: empty-note 
---
<% const yesterday = new Date(new Date().setDate(new Date().getDate()-1)) -%>
<%- 
DataStore.calendarNoteByDate(yesterday)
	.paragraphs
		.filter((p) => p.type == "open")
		.map((p) => p.rawContent)
		.join("\n") 
%>
```

### Move yesterday's tasks to today and mark them as scheduled

The following code is slightly more complicated than the first because it also marks the open tasks which we moved to today as scheduled in yesterday's note, so they don't stay open. Additionally, you could open yesterday's note in a split view for reviewing it along with today's note. See a code example above.

```
---
title: Move yesterday's tasks
type: empty-note 
---
<% const yesterday = new Date(new Date().setDate(new Date().getDate()-1)) -%>
<% const note = DataStore.calendarNoteByDate(yesterday) -%>
<% 
	// Get the open tasks and empty lines in between
	const openTasks = note.paragraphs
		.filter(p => ["open", "empty"].includes(p.type)) 
-%>
**Moved from [[yesterday]]**
<%- 
	// Convert the paragraphs into plain text separated by linebreaks
	openTasks.map((p) => p.rawContent).join("\n") 
%>
<% 
	// Mark the open tasks as scheduled
	openTasks.filter(p => p.type == "open")
		.forEach((p, i) => { p.type = "scheduled" }); 
-%>
<% 
	// Update yesterday's note with the changes made above
	note.updateParagraphs(openTasks) 
-%>
```

Here is how it works in the end (the template is opened side-by-side with the daily note, so you can see how it works):

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62991813a2c316231c2026e5/file-Vo9shYkTAW.gif)

*Last updated: 2025-05-18*
