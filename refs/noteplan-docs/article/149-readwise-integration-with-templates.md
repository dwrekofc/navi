# Readwise Integration (Template)

Using templates you can load random Readwise highlights into your daily notes. For example, load a highlight every day into your daily notes to start the day.

[Readwise](https://readwise.io) is a service that lets you upload your book highlights and review them.

Before you can use the template below you need to look up your access token and insert it into your template. You can look up your token here: [Readwise Access Token](https://readwise.io/access_token) (you need to be logged into your account). Then replace `<Your Access Token Here>` below with your token.

Here's the template:

```
---
title: Readwise Highlight
type: empty-note
documentation: https://help.noteplan.co/article/136-templates
---
<% const accToken = '<Your Access Token Here>' -%>
<%
const re = await fetch("https://readwise.io/api/v2/highlights?page_size=1&page=" + Math.floor(Math.random() * 1000), {
	method: 'GET',
	contentType: 'application/json',
	headers: {
		'Authorization': 'Token ' + accToken
	}
})
-%>
<% const parsed = JSON.parse(re) -%>
### Readwise Highlight
<%- parsed.detail != undefined ? "Your access token is not set or invalid, please create it [here](https://readwise.io/access_token) and insert it into the template." : parsed.results[0].text %>
```

How it looks like:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/629a3178a2c316231c202aff/file-fdmxHJeCew.gif)

### How to copy this template

1. After looking up your access token and saving it somewhere...
2. Copy the template text above.
3. Create a new template under "Smart Folders" -> "Templates" in your sidebar.
4. Select all and paste the template.
5. Replace `<Your Access Token Here>` with your access token.
6. You are ready to use the template now! Head over to an empty daily note and click on the "Insert Template" button.

## Highlights with author

This template is more complicated, but it also fetches the author of your highlight:

```
---
title: Readwise w/ Author & Source
type: empty-note 
---
<% const authToken = 'TOKEN' -%>
<% let url = "https://readwise.io/api/v2/highlights?page_size=1&page=" + Math.floor(Math.random() * 1000) -%>
<% const rwHResult = await fetch(url, { method: 'GET', contentType: 'application/json', headers: { 'Authorization': 'Token ' + authToken } }) -%>
<% var rwHParsed = null -%>
<% try { -%>
<% rwHParsed = JSON.parse(rwHResult) -%>
<% } catch (error) { -%>
<%   console.log(`${error.toString()}\n\n${url}\n\n${rwHResult}`) -%>
<% } -%>
<% console.log(JSON.stringify(rwHParsed)) %>
<% const rwText = (!rwHParsed || rwHParsed.detail) ? "Your authentication token is not set or invalid" : rwHParsed.results[0].text -%>
<% const rwBookID = (!rwHParsed || rwHParsed.detail) ? "Your authentication token is not set or invalid" : rwHParsed.results[0].book_id -%>
<% url = "https://readwise.io/api/v2/books/" + rwBookID -%>
<% const rwBResult = await fetch(url, { method: 'GET', contentType: 'application/json', headers: { 'Authorization': 'Token ' + authToken } }) -%>
<% var rwBParsed = null -%>
<% try { -%>
<% rwBParsed = JSON.parse(rwBResult) -%>
<% } catch (error) { -%>
<%   console.log(`${error.toString()}\n\n${url}\n\n${rwBResult}`) -%>
<% } -%>
<% const rwAuthor = !rwBParsed?.author ? "(unknown author)" : "-- " + rwBParsed.author -%>
<% const rwSource = rwBParsed?.title ? " (" + rwBParsed.title + ")" : "" -%>
### Readwise Highlight
> <%- rwText %> <%- rwAuthor -%><%- rwSource %>
```

*(Contributed by David @dwertheimer and Jonathan @jgclark in our *[Discord community](https://discord.gg/D4268MT)*)*

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/629b9cb65732000792521874/file-DCJuGLY87R.png)

*Last updated: 2025-05-18*
