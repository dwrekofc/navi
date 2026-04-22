# Meeting Notes

Create meeting notes from events in your calendar. Meeting notes use templates to add information from the event, and link the note to the event. This allows you to open the meeting note from the event, without having to search for it in the folders.

## How to create a Meeting Note?

**macOS**
Click on an event in the timeline in the right sidebar > "Create New Note" > Select a template

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787d69c0d82f5250c607da2/file-k6SIGsgd7f.png)

*If you don't have any templates, there will be an option to create sample templates in this menu. You can edit the sample templates later.*

**iOS**
Events appear in a list at the top of your daily notes.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787d88d62b4c5731439244b/file-cvDAy3ibas.png)

Long-press an event to open a drop down menu where you can tap on "Create New Note" and select a template.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787d92be54fb237c496db52/file-6H0em4WnR3.png)

Learn more about Meeting Notes in this video:

## Customize Meeting Note Templates

Modify the default templates or create completely new templates to change where and how your new meeting note should be added in NotePlan and which content they should generate. You can find the templates in the special "Templates" folder in the left sidebar under "Smart Folders".

Click on any template to modify it or right-click "Templates" to create a new template or to generate the sample templates.

**Note**: A Meeting Note template must contain the tag `<%- calendarItemLink %>` at the very least. And the frontmatter needs to contain `meeting-note` as type.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62632ce793a48c4448334bda/file-oyPlwMgD9Z.png)

## Templates

Using templates, you can automate not only the content of your meeting notes but also where they should be added. A template consists of two parts:

1. **Frontmatter** = metadata controlling how the note will be created
2. **Body** = future note content using placeholders

### Frontmatter

You can use the following parameters in the Frontmatter of your template. Frontmatter is the metadata of your template and it's located at the top between a starting and ending "---" (three dashes, displayed as a blue line in NotePlan).

**Note**: It has to be the first section in your note, it can not be preceded by a title or any other text.

Frontmatter options:

- Use `title: Your template title` to name your template.
- Use `folder: /path/to/folder` to define a specific folder for the new meeting note.
- Use `folder: <select>` to show a prompt where you can select a folder.
- Use `folder: <current>` to use the same folder where the currently opened note is located.
- Use `append: note title` to append the template content to an existing note (will be created if it doesn't exist).
- Use `prepend: note title` to prepend the template content to an existing note (will be created if it doesn't exist).
- Use `append: <select>` or `prepend: <select>` to show a prompt where you can select the note.
- Use `append: <current>` or `prepend: <current>` to append the meeting note to the currently opened note.
- Use `cursor: <current>` to insert the meeting note at the cursor position.

Make sure to add `type: meeting-note` to your Frontmatter. NotePlan uses this to filter your templates and shows only the relevant ones in the right places.

### Body

The body contains the text you want to see in your meeting notes later. You will also find templating tags that serve as placeholders to fill in the information of the selected event and you can add more templating tags (see [here](136-templates.md)). A tag starts with `<%-` and ends with `%>`. To create a link between a note and an event you need the tag `<%- calendarItemLink %>` at the very least.

Tags can also be used in the frontmatter to make the folder more dynamic for example: `folder: "📅 Meetings/<%- eventDate('MMM YY') %>"` would create the note in the folder "📅 Meetings" with the subfolder named after the month and year of the event.

You can use the following special event tags in the body and frontmatter of your templates (they have to be enclosed in `<%-` and `%>`):

- **eventTitle** = The title of the selected calendar event.
- **eventAttendees** = Comma separated list of all attendees of this event (names or emails as email links).
- **eventAttendeeNames** = Comma separated list of all attendees of this event (names or emails as plain text, available from v3.5.2).
- **calendarItemLink** = The link to this event, this has to be added to link a note to an event.
- **eventDate('MMM Do YY')** = The date of the event, you can modify the [format](https://momentjscom.readthedocs.io/en/latest/moment/04-displaying/01-format/) of the date.
- **eventEndDate('MMM Do YY')** = The end date of the event, you can modify the [format](https://momentjscom.readthedocs.io/en/latest/moment/04-displaying/01-format/) of the date.
- **eventLink** = URL which is optionally added to events, like the link to the zoom call.
- **eventNotes** = The text inside the notes field of the event.
- **eventLocation** = Location of the event.
- **eventCalendar** = Calendar name of the event.

### Example 1 - Standard Meeting Note

You can use the following example as a base. Simply copy the contents below and paste them into a new template (right-click on the "Templates" folder and create a new one). Then modify the template: remove existing tags or add new tags to shape the meeting note you need for your workflow.

```
---
title: Standard Meeting Note
folder: 30 - Resources/30.2 - Quick Notes/Meetings
type: meeting-note
---
# <%- eventTitle %> - <%- eventDate('MMM Do YY') %>
**Event:** <%- calendarItemLink %>
**Link:** <%- eventLink %>
**Attendees:** <%- eventAttendees %>
**Event Notes:** 
> <%- eventNotes %>
---

## Agenda
- 

## Meeting Minutes
-
```

In this example, we are creating a new meeting note in the folder `30 - Resources/30.2 - Quick Notes/Meetings`.

### Example 2 - Monthly Meeting Note

If you struggle with a lot of meeting notes one way to get some control over your system is to file the meeting notes in monthly folders automatically using the following template. You can modify the format the folder is named or even make it weekly instead of monthly. Find out more about date formatting [here](https://momentjscom.readthedocs.io/en/latest/moment/04-displaying/01-format/) (change the text inside eventDate(...)).

```
---
title: Monthly Folder
type: meeting-note
folder: "Meetings/<%- eventDate('MMM YY') %>"
documentation: https://help.noteplan.co/article/134-meeting-notes
---
# <%- eventTitle %> - <%- eventDate('Do') %>
**Event:** <%- calendarItemLink %>
**Attendees:** <%- eventAttendees %>

---

## Agenda
- 

## Meeting Minutes
-
```

In this example, we are creating a new meeting note in the folder `Meeting Notes/May 22`.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/627e6c75b2de5178f888380f/file-HtOetZdCDO.png)

### Example 3 - Link to Send All Attendees an Email

Once the meeting is over and you have finished writing the meeting minutes you might want to send all attendees an email. Use the following template which generates a link that opens your default mail app with the attendees' email addresses prefilled, so you are ready to send the meeting note. You can create an RTF, PDF, or publish the note and share the link in that email.

This template is also an example of how you can mix Javascript code with template tags to create more dynamic templates:

```
---
title: Meeting Note w/ Email Link
type: meeting-note
documentation: https://help.noteplan.co/article/134-meeting-notes
---
# <%- eventTitle %> - <%- eventDate('MMM Do YY') %>
**Event:** <%- calendarItemLink %>
**Attendees:** 
<%- eventAttendees %>
<% const emails = eventAttendees.match(/([a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+\.[a-zA-Z0-9_-]+)/gi) -%>
<% const uniqueEmails = [...(new Set(emails))].join(",") -%>
→ [✉️ Email Attendees](mailto:<%- uniqueEmails %>?subject=<%- encodeURIComponent(eventTitle) %>)
---

## Agenda
- 

## Meeting Minutes
-
```

This template is a [community](https://discord.gg/D4268MT) contribution by David (@dwertheimer).

## Contributions

Thanks to the plugin community, with special thanks to Mike (aka [@codedungeon](https://mobile.twitter.com/codedungeon)) for creating the templating engine on which the Meeting Notes are based! You can contribute with your ideas, reports, or feedback by visiting our [Discord](https://discord.gg/D4268MT) community.

*Last updated: 2026-04-03*
