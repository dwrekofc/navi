# Todoist Integration - Get Active Tasks (Template)

You can integrate Todoist into NotePlan using the following template which downloads all your active tasks and prints them in a list into your note. Make sure to look up your API Token and replace it with "SECRET" in the template.

You can find your API Token in your [integration settings](https://todoist.com/app/settings/integrations) of your account. Scroll to the bottom and copy the token text:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/629f7d4f57320007925222e9/file-uYvGS4tYsH.png)

## Template

This template is a simple example of how you can get all your active tasks from Todoist into NotePlan. Feel free to extend it and get more information out as needed. If you check off tasks in NotePlan they will **not** be synced back to Todoist in this example.

Add this template to NotePlan by copying the text below, then create a new template in NotePlan's sidebar under "Templates", select all (CMD+A), and paste everything.

```
---
title: Todoist Integration (Get Active Tasks)
type: empty-note 
---
<% const apiToken = 'SECRET' -%>
<%
const re = await fetch("https://api.todoist.com/api/v1/tasks", {
  method: 'GET',
  headers: {
    'Authorization': 'Bearer ' + apiToken,
    'Content-Type': 'application/json'
  }
})
-%>

<% const data = (re == "Forbidden" ? null : JSON.parse(re)) -%>
<% const items = data ? data.results : null -%>

<%- items == undefined ? "Your Todoist API token is invalid, look it up on todoist.com → settings → Integrations → scroll to the bottom and copy the API Token and replace it with 'SECRET' in the template" : items.map(item => "* [ ] " + item.content + (item.due ? " >" + item.due.date.substring(0,10) : "")).join("\n") %>
```

This template will load all your active tasks into the current note:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/629f7dc692cb8c175b46a7bd/file-xYuBVDdm5z.png)

Becomes:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/629f7e8292cb8c175b46a7c5/file-2tyszDjIQu.gif)

*Last updated: 2026-02-13*
