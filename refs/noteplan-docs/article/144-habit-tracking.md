# Habit Tracking

***Note**: [Tables are available now! Build your habit tracker using tables](189-tables.md).*

If you are trying to build one or more healthy habits into your daily or weekly routines you want to track your progress visually.

One simple way is to report your progress in your own words or by using emojis in your daily notes. Using daily notes you can go back and review other days individually, but you won't get a full overview. What you need is a habit table that lives in an individual note (not in the daily notes).

## Habit Table

In NotePlan you can build simple tables using the tab key to separate columns from each other and create consistent spacing. Using this "tool" you can jot down the days vertically on the left and your habits at the top horizontally. Create a table for each individual month for example:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6292437f02b8fd3b945f84be/file-6TaVsoD33Z.jpg)

In the example above the habits are listed in their short form at the top and progress is captured using either a check or stop emoji. You can mix in other emojis that can have different meanings for you.

Here's how you can type up a simple table using tabbing:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62924b5c92cb8c175b467d6d/file-QFLuz3VTzL.gif)

## Templates

Instead of typing up the full table, you can use one of the templates below. Copy the template text and create a new note in your “Templates” folder (below “Smart Folders”), then select all and paste the template.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6293cf2ba2c316231c201020/file-VZZFAYoYWA.gif)

Once the template is added, you can insert a template into a note by creating a new note. Before writing anything into that note click on the “Insert Template” button in the middle of the page.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6293cf8302b8fd3b945f8659/file-zaznJf1Htu.gif)

### Monthly

This template contains a table with 31 days in the left column. Not every month has that many days, so you need to manually delete some from the end as appropriate. It attaches the month and year to the note title when you insert the template. Modify the template as needed.
```
---
title: Monthly Habit Tracker
type: empty-note 
---
# Habit Tracker - <%- date.now("MMMM YYYY") %>
Instructions: *Delete extra days at the end of the table to match it with the actual days this month contains. Add or remove habits as needed. This example starts with three habits which you are prompted to enter when the template is inserted. Every day paste ✅ or 👎 into the table right of the appropriate day. At one glance you can see if you are keeping your streak.* [Learn more](https://help.noteplan.co/article/144-habit-tracking)

- Track Habits >today
Status: ✅ = Done, 👎 = Missed, 🟠 = Average, 🟢 = Good, 🔴 = Bad

Note: *Add or remove tabs as needed to line up the columns*
*Day/Habit*	`<%- prompt('Short name for habit 1') %>`		`<%- prompt('Short name for habit 2') %>`		`<%- prompt('Short name for habit 3') %>`
**01**			✅		✅		
**02**			
**03**			
**04**			
**05**			
**06**			
**07**			
**08**			
**09**			
**10**			
**11**			
**12**			
**13**			
**14**			
**15**			
**16**			
**17**			
**18**			
**19**			
**20**			
**21**			
**22**			
**23**			
**24**			
**25**			
**26**			
**27**			
**28**			
**29**			
**30**			
**31**
	
```

### Weekly

An alternative option to having a monthly overview is to track your habits on a weekly basis and either save a note for every week or copy and paste a new week below the old one. This template automatically generates a full week for the current week. If you want to create a table for the next week, you need to wait until it begins or copy it manually.
```
---
title: Weekly Habit Tracker
type: empty-note 
---
# Habit Tracker W<%- date.weekNumber() %>
Instructions: *With this note you can track a habit for one week. Either create a new note for the next week and insert the template once the week started or copy and paste another week into this note. Add or remove habits as needed. Every day paste ✅ or 👎 into the table right of the appropriate day. At one glance you can see if you are keeping your streak.* [Learn more](https://help.noteplan.co/article/144-habit-tracking)

- Track Habits >today
Status: ✅ = Done, 👎 = Missed, 🟠 = Average, 🟢 = Good, 🔴 = Bad

Month: `<%- date.now("MMMM YYYY") %>`
Timeframe: `<%- date.startOfWeek("Do") %> - <%- date.startOfWeek("Do", '', 6) %>`

Note: *Add or remove tabs as needed to line up the columns*
*Day/Habit*	`<%- prompt('Short name for habit 1') %>`	`<%- prompt('Short name for habit 2') %>`	`<%- prompt('Short name for habit 3') %>`
**Sun**			✅		
**Mon**		
**Tue**			
**Wed**		
**Thu**			
**Fri**			
**Sat**			

*Once the next week started, create a new note and insert this template.*
	
```

### Monthly with Links to Daily Notes

Generates a habit tracking note with links to the daily notes in the left column (thanks to [@suppersready](https://discord.com/channels/763107030223290449/963950027946999828/1000068799304826880) for sharing it on Discord).

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62f6934c80fd5a31e7ad1cf6/file-EVd8dUo1Sf.png)Template:

```
---
title: Habits Template
type: empty-note 
---
<% 
var habits = [`💤`, `💧`, `☀️`, `🍎`, `⌛`, `📖`, `🏋️`, `☠️`, `🦷`, `☑️`];

var weekDays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

var d = new Date();
var month = d.getMonth();
var year = d.getFullYear();

function getDaysInMonth(month, year) {
  var date = new Date(year, month, 1);
  var days = [];
  while (date.getMonth() === month) {
    days.push(new Date(date));
    date.setDate(date.getDate() + 1);
  }
  return days;
}

var days = getDaysInMonth(month, year);

var header = `# ${d.toLocaleString("default", { month: "long" })} ${year} Habits
Status: 🟠 = Average, 🟢 = Good, 🔴 = Bad, 🤔 = Pending

`;

var table = "                 ";

habits.forEach((h) => {
  table += `\`${h}\`    `;
});

table += "\n";

days.forEach((d) => {
  table += `[**${weekDays[d.getDay()]} ${d
    .getDate()
    .toString()
    .padStart(2, "0")}**](noteplan://x-callback-url/openNote?noteDate=${d
    .toLocaleDateString("sv")
    .replace(/-/g, "")})`;
  habits.forEach((h) => {
    table += `    \`🤔\``;
  });
  table += "\n";
});

var render = header + table;

-%>
<%- render %>
```

*Last updated: 2023-03-07*
