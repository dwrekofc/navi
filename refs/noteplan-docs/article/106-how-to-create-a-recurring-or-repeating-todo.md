# How to create a repeating task

A recurring (or repeating) task appears in multiple notes, so you don't have to create it each time. There are three methods to create a recurring task:

1. By copying the task across multiple notes (through the task menu > Repeat).
2. By using an [auto-insert template](229-auto-insert-templates.md).
3. By using [Reminders](https://noteplan.co/changelog/v3.11.2-improved-reminders-integration).

## 1. Copy Repeating Task

A repeating task is copied across future daily notes with an `@repeat(occurrence/total)` tag which indicates the occurrence of this instance of the task like 5/10. The last occurrence indicates the final one as well, so you know when to create a new series.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e3d90d82f5250c607dfc/file-F6JlfRBnzy.png)

If you repeat a task from the daily notes, it will use the date of that daily note as the start, unless the task contains a date tag ( `>YYYY-MM-DD`). A task repeated from a project note will use today's date unless it has a **date tag**.

To **delete** a repeating task and all it's repetitions, delete the first occurrence and NotePlan will ask you if you want to delete future occurrences as well.

**macOS
**Select one or multiple tasks, then open the schedule menu by hovering over the task and clicking on the arrow icon or using **⇧+⌘+D** keys. This opens a menu where you can switch at the top from `Schedule` to `Repeat`. Setup your preferences and hit the "Repeat" button at the bottom of the dialog:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e39862b4c573143924b6/file-LD1tz82qbd.gif)**iOS**
Select the task(s) by placing the cursor into the text of the task. This will open the keyboard at the bottom with the toolbar above it. Tap on the task menu on the left (checkmark with + button). Here you can select "Schedule", which opens a new view where you can select "Repeat" at the top.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e4ecc3601717f7d1df96/file-1YTbSXtrhB.gif)

**Deleting a repeated task**
Select the task and hit delete on macOS or iOS, then confirm the dialog that opens with "yes". **![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e63262b4c573143924ca/file-BZUyNnJ1xh.gif)**

## 2. Auto-Insert Templates

See full documentation [here](229-auto-insert-templates.md).

NotePlan can automatically insert templates into daily notes when you open them for the first time (using flexible recurrence rules). This way recurring tasks are added to your notes as you create the daily note (i.e. they are not created in advance).

Create a template by right-clicking (long-press on iOS) the "Templates" folder in the left sidebar, then "New Template":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e6b86902d90cc40c77f5/file-WTxyLaNuFZ.png)

At the top right of the template you will see a calendar-clock icon. Tap/Click on this to see the recurrence settings. Setup the recurrence you need and hit "update":

**macOS**

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e7a4e54fb237c496dbbb/file-ID8czRKcL8.gif)

**iOS![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e88562b4c573143924da/file-q7RB31mj51.gif)**

## **3. Reminders**

Using Apple's Reminders you can create a reminder and setup a recurrence rule. Reminders are integrated with NotePlan, so you will see the reminder in the right sidebar in the timeline view, and in the "Reminders" filter view, which allows you to drag the reminder into your daily note:

**macOS**
Enable the "Reminders" view in the right sidebar by clicking on "Timeline" drop down, then select "Reminders".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e9b3ad98cf53efde0b39/file-xgELnfjK4J.gif)

Here you can drag the recurring reminder into your daily note. NotePlan will complete the recurrence in the Reminders app, so it will update for the next date.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787e9f56902d90cc40c7807/file-A5FzBErd6F.gif)

You can also drag it into a regular (non-calendar) note and it will attach the date as well.

**iOS**
Open the timeline at the bottom of the screen by tapping on the calendar-clock icon. Then you can switch from "Timeline" to "Reminders" and drag the reminder into the note.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6787eab90d82f5250c607e25/file-dU4Q68vfw2.gif)

##

*Last updated: 2025-01-15*
