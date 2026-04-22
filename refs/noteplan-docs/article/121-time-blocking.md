# Time Blocking

Time Blocking means you assign a dedicated timeframe in your day to work on a task. This helps you to keep the balance between staying focused and spending too much time on a task by getting into a "rabbit hole" = working too long on a task.

NotePlan supports Time Blocking by creating visual blocks of time in your timeline on the right, which hosts normally your events and reminders.

The easiest way to add a Time Block is to drag a checklist, task or bullet into the timeline (on Mac in the right sidebar, on iOS you need to open the timeline at the bottom tapping on the calendar + clock icon).

## Time Blocking on Mac

## Time Blocking on iPhone and iPad

## How to create a Time Block?

You can define a time block by adding a start and optional end time to bullets, tasks, checklists, or headings (it won't work with plain text). The easiest way is by dragging the line from the editor into the timeline in the sidebar on the right:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6408ebc6b550cb3e694bec77/file-BznOR32pVI.gif)NotePlan detects the time in the sentences using natural language processing. You can use 12h format (e.g. "5:00pm") and 24h format (e.g. "14:00"). The more detailed you are, the easier it is for NotePlan to detect the time. For example, NotePlan won't detect the time if you just write "15", but "15:00" works. However, the best approach is to drag the task into the timeline, so NotePlan itself adds the time as it is expected.

## Send Notifications

NotePlan can show a notification when the Time Block starts. Here's how you can enable notifications.

**macOS
**Click on the settings icon right of "Timeline" in the right sidebar

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678d1a7e77ccd8346af8e3bf/file-2lwOuUqD4y.png)**iOS**
Open the timeline in the bottom by tapping on the calendar + clock icon, then on the settings icon on the right.

## ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678d1af6a1b1d403c467b9a8/file-PZt1SvuItP.png)Sync Time Block to your Calendar

If you want to see your timeblocked tasks in other calendar apps, on your Apple Watch, etc. you can enable that NotePlan syncs your Time Blocks to your calendar of choice.

Learn more [here](217-sync-timeblocks-with-your-calendar.md).

## Prevent accidental Timeblocks

****To prevent accidental timeblocks in lines that contain text which is similar to a time, you can add a filter in the settings under Todo. We recommend using a time emoji for this, like ⏱️.   ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6408eb3df49b5b3acbd2f155/file-55jKcUwJ8r.png)
## Creating events instead of Time Blocks

Time Blocks don't appear in your calendar (unless you enable [the sync](217-sync-timeblocks-with-your-calendar.md)), but you can create events instead of just a visual block by selecting "Add Event" in the context menu that appears when you drop a task into the timeline. It will create an event that you will see in other calendar apps. In NotePlan the text will be replaced with an embedded event. You can update the event by clicking on it or on the event in the timeline and editing the details (switch to another note to get the update in the editor).

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6408ecd9b550cb3e694bec7c/file-HK4ZTgJY1x.gif)

## Best Practices

Why bullets, checklists, and headings in addition to tasks?

Defining time frames that are too tight or too loose has both problems. If your time frames are too tight (which means planning out down to every 15min) your day can quickly get very crowded and complicated. For example, if you take too long with just one task everything else also slips. On the other hand, if you have just two blocks of time for the whole day, you probably don't need to write that down. That's where having a Time Block defined inside a heading comes in handy.
#### Headings

You can block out some time for a specific type of task and then list them below the heading. For example:
```
## 09:00 - 11:00 Communications
* Reply to emails
* Reply to messages 
* Check to tweets
```

#### Lists and Bullets

Instead of headings, you can also use simple bullets or numbered lists. Similar to headings, you can define a general theme for a Time Block, then add tasks indented below it or just use the bullets for your Time Blocks as guard rails for your day:
```
- 09:00 - 11:00 Communications
	* Reply to emails
	* Check tweets
```

## Daily Notes vs Regular Notes

If you type a Time Block directly into the daily notes, the date is already implied and the block will be displayed for that day. But if you are inside a regular (project) note, you need to define the date using date tags:  `* 5:00pm Reply to emails >2022-01-10`  Then it will be displayed in the daily note as well.
## Community Plugins

[Read this](65-commandbar-plugins.md) first to learn how to install plugins.
- With the plugin ["Event Helpers"](https://github.com/NotePlan/plugins/tree/main/jgclark.EventHelpers) you can create calendar events from your Time Blocks (with the `/time blocks to calendar` command)
- Using ["Event Automations"](https://github.com/NotePlan/plugins/tree/main/dwertheimer.EventAutomations) you can create Time Blocks from tasks which are scheduled to today using the `>today` tag (with the `/atb` command).

## Themes

You can change the color of the Time Blocks by adding the following attribute in a custom theme to "editor":
```
"timeBlockColor": "#d87001",
```

*Last updated: 2025-01-19*
