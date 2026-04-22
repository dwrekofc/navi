# MeisterTask Integration (Template) - Get All Open Tasks

You can integrate MeisterTask into NotePlan using the following template which downloads all your open tasks and adds to your note. Make sure to look up your API Token and replace it with "SECRET" in the template.

You can find your API Token in the [API settings](https://mindmeister.com/api) of your account (you need to log in). Scroll to the bottom, create a personal access token and copy the token text:

## ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62de712179bb3605c394aa18/file-rvBbeL0GmR.png)Template

This is template is a very simple example of how you can get all your open tasks from MeisterTask into NotePlan. Feel free to extend it and get more information out as needed. If you check off tasks in NotePlan they will **not** be synced back to MeisterTask in this example.

Add this template to NotePlan by copying the text below, then create a new template in NotePlan's sidebar under "Templates", select all (CMD+A), and paste everything.

```
---
title: MeisterTask-Tasks
type: empty-note 
---
### Open Tasks from MeisterTask
<% const apiToken = 'SECRET' -%>
<%
const re = await fetch("https://www.meistertask.com/api/tasks?assigned_to_me=1&status=open", {
    method: 'GET',
    contentType: 'application/json',
    headers: {
        'Authorization': 'Bearer ' + apiToken
    }
})
-%>
<% const parsed = (re == "Forbidden" ? null : JSON.parse(re)) -%>
<%- parsed == undefined ? "Your MeisterTask API token is invalid, look it up on mindmeister.com/api → scroll to the bottom and copy (or create) the personal access Token (give permissions to 'meistertask') and replace it with 'SECRET' in the template" : parsed.map(p => "* [ ] " + p.name + " [Details](https://www.meistertask.com/app/task/" + p.token + ")").join("\n") %>
```

## ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62de736c86b3a97442470617/file-r56ZUhkcRl.gif)

## Contributions

Thanks to the community for providing this example ( [@jongjo](https://discord.com/channels/763107030223290449/774800048738271313/993242506655449098)) and the underlying templating functionality (@codedungeon and the plugin team as a whole). Join our [discord community](https://discord.gg/D4268MT) and visit our [templating channel](https://discord.gg/AEBJgtwqJ9) to learn more.

*Last updated: 2025-05-18*
