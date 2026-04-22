# Templates

If you are using the same structure in your notes or need some automation to fill certain information you can use templates to do most of the work for you, so you don't need to copy & paste text over and over again.
- [Getting Started](#getting-started)
- [Template Structure](#template-structure)
- [Examples](#examples)
- [Use Cases](#use-cases)
- [Advanced](#advanced)

## Getting Started

To get started with your first template, open an **empty** daily or regular (project) note and you will see an "Insert Template" button on iOS and Mac which opens a context menu with the available templates. If you have no templates yet, NotePlan will suggest you create a few samples (click on "Create sample templates"), then try again.

**macOS**

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6297ed33e1d2cf0eac00d1b3/file-TGbSS6rKnx.png)

**iOS
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678977563ca88966837117b8/file-I6MNuYgu9m.png)**

### How to create a new template?

You can see the list of available templates in the new "Templates" folder in the sidebar under "Smart Folders". That's where you can also create new templates or edit/delete existing ones.

**macOS
**Right-click on "Templates" to create a new one or click on the chevron button to expand the folder and see existing templates for editing.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678977c07f0b2d5f2ab7ba25/file-3XDDNgIDtS.png)**iOS**
Long-press "Templates" in the sidebar to open a menu where you can create a new template (tap on "New Note" in older versions): ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678979c5fea89162338ebe6e/file-PXASE0vjF8.png)

To insert templates into existing notes, see [here](147-insert-templates-into-existing-notes.md).
You can also select a template to be auto-inserted. [Read more about this here](229-auto-insert-templates.md).

Watch an introduction to Templates & Meeting Notes by Stacey:

##

## Template Structure

A template consists of two parts:

1. **Properties** (optional) = metadata like title, type, and other information
2. **Body** = future note content using placeholder tags

###

### Properties

The metadata defining the type, title and other values.

It's optional but recommended because here you can define the title and type of the template. For example, the type `empty-note` will make the template appear as an option for empty notes (when you click on the "Insert Template" button). And the type `meeting-note` tells NotePlan to show this template as an option for creating meeting notes.

All template types:

- `empty-note` = Shows the template as an option in empty notes (when you click on the "Insert Template" button).
- `meeting-note` = Shows the template as an option in meeting notes (right-click / long-press an event for example).
- `calendar-note` = Shows the template only in (daily) calendar notes (available in v3.5.2).
- `project-note` = Shows the template only in (regular) project notes (available in v3.5.2).

###

### Tags

You can use various tags in templates to display dynamic information in your notes from something as simple as printing a date & time to more complicated prompts, web services, and conditionals.

In most cases, a tag starts with `<%-` and ends with `%>` such as

`<%- date.now("Do MMMM YYYY") %>`

which prints the current date in the given format.

##

## Examples

Following find a list of examples showing off some of the most common tags and ideas. These examples are also included into NotePlan and you will be prompted to create them unless you already got templates. Feel free to copy & paste the templates from here into NotePlan by creating a new template in the Templates folder.

###

### Daily Note w/ Affirmations

This template has three headings as a suggestion where you can split your tasks into the primary focus, morning (high energy) and afternoon (low energy) tasks. However, the "Journal" section has the interesting templating features:

- `<%- date.now("Do MMMM YYYY") %>` - prints current date with day, month and year
- If you want to add or subtract days, you can append another variable, like `<%- date.now("Do MMMM YYYY", -7) %>`
- `<%- web.weather() %>` - prints local weather data
- `<%- web.services('affirmation','affirmation') %>` - prints a random affirmation
- `<%- web.advice() %>` - prints a random advice
- `<%- web.quote() %>` - prints a random quote

The tags containing "web" are accessing websites to pull the information from the internet and therefore need an internet connection. It can take a few seconds for the data to load.

```
## Primary Focus
- First:
- Second:

## Morning Tasks
- 

## Afternoon Tasks
- 

---
## Journal
*<%- np.date.now("Do MMMM YYYY") %>*
Weather: <%- web.weather() %>

Affirmation:
> "<%- web.services('affirmation','affirmation') %>"
Advice:
> "<%- web.advice() %>"
Quote:
> "<%- web.quote() %>"
```

#### Output

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/626312a593a48c4448334b74/file-eKgwwVWJC7.png)

####

### Daily Note w/ Prompts & Events

This example uses three headings: Primary Focus (for the most important goal of the day), Tasks (you need to do to reach it) and Events (where all your calendar events for that day get listed).

**Note: You need to install the Event Helper plugin to make the event listing work, otherwise it prints an error. **

The tags used in this template are:

- `<%- prompt('What's your mood today?',['🙂','😐','😕']) %>` - a prompt will ask you to select one of the three emojis and prints the selected one
- `<%- prompt('Most important task today?') %>` - a prompt will ask you to enter a text value
- `<%- listTodaysEvents({template:"- *|START|*-*|END|*: *|TITLE|*",allday_template:"- *|TITLE|*"}) %> ` - prints today's events in the given format (needs Event Helpers installed)

```
*Mood: <%- prompt('What's your mood today?',['🙂','😐','😕']) %>*

## Primary Focus
* <%- prompt('Most important task today?') %>

## Tasks
* 

## Events
<%- listTodaysEvents({template:"- *|START|*-*|END|*: *|TITLE|*",allday_template:"- *|TITLE|*"}) %>
```

##

#### Output

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/626315107488cf771e51c173/file-i6YL9byA8n.png)

## Use Cases

- [Habit Tracking](144-habit-tracking.md)
- [Travel Checklist](145-travel-checklist.md)
- [Readwise Integration](149-readwise-integration-with-templates.md)
- [Jira Integration - Get Active Tickets (Template)](157-jira-integration-get-active-tickets-template.md)
- [Todoist Integration - Get Active Tasks (Template)](151-todoist-integration-get-active-tasks-template.md)
- [Log Data to Another Note in Templates](152-log-data-to-another-note-in-templates.md)
- [Use AI prompts in your templates](233-ai-prompts-in-templates.md)

## Advanced

- [Add web services to your templates (such as the bitcoin quote)](146-add-a-webservice-to-templates.md)
- [Run JavaScript code in your templates](148-run-javascript-code-in-your-templates.md)
- [Embed JavaScript code as code block into your template](254-embedded-template-code-templatejs.md)
- [Dates in Templates](150-dates-in-templates.md)
- [Template Prompts (Dialogs)](261-template-prompts-dialogs.md)
- [Conditionals](266-conditionals-if-else.md)

## Learn more

- [Full Templating Documentation](https://noteplan.co/templates/docs)
- [Learn about Meeting Notes templates.](134-meeting-notes.md)

## Contributions

Thanks to the plugin community, with special thanks to Mike (aka [@codedungeon](https://mobile.twitter.com/codedungeon)) for creating this templating engine and the plugins based on it! You can contribute with your ideas, reports, feedback or by working directly on plugins by visiting our [Discord](https://discord.gg/D4268MT) community or our [GitHub repository](https://github.com/NotePlan/plugins).

*Last updated: 2025-05-28*
