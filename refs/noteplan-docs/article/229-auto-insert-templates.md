# Auto-Insert Templates (Recurring Templates)

Select a template to be automatically inserted into new calendar notes. This works for daily, weekly, monthly, quarterly, or yearly. The feature activates when you open a new note or drag a task into a new day. However, it doesn't apply to existing notes (with or without content).   *"New note" or "new day" means in this context that the calendar note file doesn't exist yet in the calendar folder. If you have never opened and edited a calendar note, nor dragged something into it, it most likely doesn't exist yet. See this *[article](244-repeating-tasks-and-auto-inserting-templates-is-not-working-for-me.md)* to learn more.*  You can choose a specific template for each type of calendar note, or open the template directly and click top right on the repeat button. NotePlan will add an entry to the frontmatter of the selected template, allowing it to identify which template to use for each note type.
## 1. Select a Template from a Calendar Note

To set up auto-insert templates:
1. Open an empty calendar note on Mac or iOS.
2. Click "Insert Template" and choose "Auto-Insert".
3. Select a template.

The chosen template will be saved for that specific calendar note type (daily, weekly, etc.). For example, the selected template will be used for all future daily notes if you're in a daily note, weekly if you are in a weekly note, etc.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66d769bae6b2356def908c9d/file-zeRrJzI40O.gif)

To un-select a template, you can follow the same process and click on the checked template again to "uncheck" it.

## **2. Setup a Template Directly**

Open the template and click on the repeat button on the top right:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66ec656444628317ef903fa7/file-tzRSxULf7X.png)

A dialog opens with a few options. Select the "Insert Into" option to "Daily" to reveal more preferences:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66ec658915d0b822330641d8/file-O9yKnKdOhL.png)

"Insert Into" means the calendar note into which the template should be auto-inserted. The most flexible is the daily notes option.

## How to stop auto-insert

1. Open the template that is auto-inserting from the templates folder in the left sidebar.
2. Open the recurrence menu (calendar with clock icon) top right of the template.
3. Select for "Insert Into" the value "None".

## ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/679a4db86845d205f0be381e/file-haaVPxZCcZ.gif)What happens if I update the template?

If you change an existing template, it will insert the updated content when you open a new calendar note. The existing calendar notes, where the template has been already inserted, won't be affected.

This way you don't need to worry about future calendar notes. They don't exist yet and the template hasn't been inserted yet.

## Frontmatter

If you need more fine-grained control over your auto-insert templates, you can edit the frontmatter in your templates directly, circumventing the user interface. Here, you can also define multiple templates for different days, or overlapping days (means multiple templates for the same date). For example, if you need one template for weekdays, but another for the weekend, you might want to use the following repeat field inside your template frontmatter:

```
repeat: note = day, freq = week, on = [mon, tue, wed, thu, fri]
```

This will select the template only for weekdays. Another template could contain the following for the weekend days:

```
repeat: note = day, freq = week, on = [sat, sun]
```

If you in addition to the previous two need something in your templates independently of the date (overlapping with the two previous), you can use:

```
repeat: note = day
```

This template will be added to the other two. You can also define the order by adding order = 1 for example.

*Note: The template type has to be empty or contain `calendar-note` or `empty-note`.*

The following fields are available behind the "repeat:" key:

- **note:** The calendar note type, means daily, weekly, etc. (values: `day`, `week`, `month`, `quarter`, `year`)
- **freq**: Frequency of repetition, means every day, every week, etc. (values: `day`, `week`, `month`, `quarter`, `year`)
- **on**: Specify the list of days for weekly, monthly, or yearly frequencies (the values have to be added as a list, means inside square brackets like `[mon, tue]`)
- For the weekly frequency use days of week: `mon`, `tue`, `wed`, `thu`, `fri`, `sat`, `sun` (example: `[sat, sun]`)
- For monthly, you can use the number of the day in a month: `1`, `2`, `3` ..., `31` (example: `[1, 2, 30]`)
- For yearly, use month-day (MM-DD) pairs: from `01-01` to `12-31` (example: `[12-24, 09-23]`)
- **order**: If you have multiple templates matching for a day, then this will determine in which order they will be added. Use a simple number, like 1, 2, 3...
- **async**: If the template is executing functions that need some time to run like "listTodaysEvents" or fetching something from the internet or prompting you to type in something, you can add "async = true", so NotePlan will wait until the template is fully processed. This is mostly needed when you run multiple templates, so they wait for each other.

A complete template looks like this for example:

```
---
repeat: note = day
title: Daily Planning
type: calendar-note
---

## Today's Goals
> 

## Tasks
```

### Asynchronous Functions in Templates

For example "listTodaysEvents" or web calls or prompts. If you are using these in your template, add "async = true" to the "repeat" line, so NotePlan waits for the template. This is only needed if you are inserting multiple templates into the same day.

*Last updated: 2025-05-18*
