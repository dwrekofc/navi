# Embedded Template Code (templatejs)

To make coding in templates easier, you can add a code block with the language "templatejs", and use all the variables that are generated in this code block inside your template. You can use JavaScript in this code block and have full access to the [JavaScript Plugin API](70-javascript-plugin-api.md), and templating modules. An embedded code block helps if you have a lot of JavaScript code like in [this example](162-meistertask-integration-template-get-all-open-tasks.md).

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/679d5efdac3b8d27c40957ee/file-c6X87YJ3ze.png)

For example:

```
---
title: Overdue Tasks
type: meeting-note, empty-note 
---
```templatejs
// Get all overdue tasks
const overdues = await DataStore.listOverdueTasks()

// Get the overdue task count
const overdueCount = overdues.length

// Get the latest overdue task
const latestTask = overdueCount > 0 ? overdues[0] : undefined

// Create a variable that holds the content if there is a latest task
let taskContent = ""
if(latestTask) {
	taskContent = latestTask.rawContent
}
```
Overdue Tasks: <%- overdueCount %>
<%- taskContent %><br>
```

This will print the amount of overdue tasks and the latest overdue task below.

[Learn here](148-run-javascript-code-in-your-templates.md) about how you can use JavaScript in general in your templates, and if you don't have that many lines of code (means you don't need a code block).

*Last updated: 2025-05-18*
