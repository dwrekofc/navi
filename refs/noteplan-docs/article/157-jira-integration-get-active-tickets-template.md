# Jira Integration - Get Active Tickets (Template)

This template will pull all tickets assigned to you with the status "To Do" and "In Progress" into the current note. You will need to do a little bit of setup before you can use this integration. You need:

1. An API Token: [Generate it here](https://id.atlassian.com/manage-profile/security/api-tokens)
2. Your user email address
3. Your organization URL, such as "[https://noteplanapp.atlassian.net](https://noteplanapp.atlassian.net/)"

Once you have the token and your email address, you need to base64 encode it in this combination: `email:api-token`. You can use any tool or website to encode it, for example, [this one](https://www.base64encode.org/).

## Template

Before you copy it into NotePlan, make sure to replace:

- the `BEARERTOKEN` with your base64 encoded `email:api-token` combination.
- And replace the `jiraURL` with the link for your organization.

```
---
title: Jira tickets
type: empty-note 
---
<% /* bearerToken is user_email:jira_acess_token base64 encoded */ -%>
<% const bearerToken = 'BEARERTOKEN' -%>
<% /* replace with your company jira url */ -%>
<% const jiraURL = "https://YOUR_ORG.atlassian.net"-%>
<%
const re = await fetch(jiraURL + "/rest/api/3/search?jql=assignee=currentuser()&fields=id,key,status,summary", {
	method: 'GET',
	contentType: 'application/json',
	headers: {
		'Authorization': 'Basic ' + bearerToken
	}
})
-%>
<% const parsed = JSON.parse(re) -%>
## Jira Tickets
### In Progress
<%- parsed == undefined ? "Your API token is invalid ": parsed.issues.filter(ticket =>  ticket.fields.status.name ===  "In Progress").map(ticket => "* [" + ticket.key + "]" + "(" + jiraURL + "browse/" + ticket.key + ") " + ticket.fields.summary) .join("\n") -%>

### Todo
<%- parsed == undefined ? "Your API token is invalid ": parsed.issues.filter(ticket =>  ticket.fields.status.name ===  "To Do").map(ticket => "- [" + ticket.key + "]" + "(" + jiraURL + "browse/" + ticket.key + ") " + ticket.fields.summary) .join("\n") -%>
```

See the integration here in action:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62b3511ca43c1525b680ced7/file-cfj0YIfFFK.gif)

## Contributions

Thanks to the community for providing this example ( [@AaronG](https://discord.com/channels/763107030223290449/963950027946999828/989198112704847965)) and the underlying templating functionality (@codedungeon and the plugin team as a whole). Join our [discord community](https://discord.gg/D4268MT) and visit our [templating channel](https://discord.gg/AEBJgtwqJ9) to learn more.

*Last updated: 2025-05-18*
