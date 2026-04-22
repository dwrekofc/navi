# Tables

NotePlan 3.8.1 introduces tables, which help you present information more elegantly. Think of habit trackers, goal trackers (OKRs), a shopping list with quantities (and prices), or any data you want to show in a more structured way.

**Update [v3.17.3](https://noteplan.co/changelog/v3.17.3): Added support for markdown formatting**

*Note: Tables are supported on macOS Big Sur+ / iOS 13+*

Learn how to create tables and get inspired by some examples:

##

## How to create tables?

On macOS, click on the `+` button when you hover over an empty line, then select "Insert Table" or type "/table" as a slash command directly without using the mouse.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63ffbbfa0064af3c7159e94b/file-EPDkUPak72.png)

On iOS, use the `⌘` menu in the toolbar above the keyboard (which appears when you edit the note), select “Insert +” and choose “Table”:

## ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63ffbc86c490cd5d5b96a989/file-pWyAvvgSn0.gif)How to add/remove rows and columns?

Add or delete rows and columns by clicking on the options button which appears if you click into a table cell. On macOS it's enough to hover the cell, and on iOS, you have to tap into a cell to see this button:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63ffbcc90064af3c7159e94d/file-8r8AoV1osz.png)

Use the arrow keys to move around cells and use the Return key to add a new row quickly (once the cursor is in the last row). You can also use tab/back-tab to move to the next/previous cell.

## How are tables saved in my notes?

NotePlan saves your notes as markdown text into your notes. You can open the plain text file and see the table, it looks like this:

```
| Account | Value |
| ------- | ----- |
| Broker | X USD |
| Bank Account 1 | X2 USD |
| Bank Account 2 | X3 USD |
| Business Account | X4 USD |
| = Total | = X + X2 + X3 + X4 USD |
```

This format is compatible with other markdown apps.

## Markdown Formatting in Tables

## What can I use tables for?

Besides organizing and structuring data, you can:

- Build a **habit tracker** and store it in your weekly notes. Every day open your weekly note to report your progress and at the end of the week review how it went.
- Build a **goal tracker** using the OKR methodology (Objectives and Key Results). Set it up for every month and track your progress against your target or set it up once for the year and revisit it monthly for check-ins.
- Track your investments in a Portfolio Tracker.
- Keep an overview of your bank accounts.

### Tables for Goal Tracking (OKR)

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/640602cf74c58f66ad0cfa68/file-pPXanoDjy4.png)You can do some simple OKR (goal) tracking using tables. Create a table with at least four columns: Key Results, Target, Progress, and Check-in. Above your table define your Objective (create multiple tables if you have more objectives).

The Objective is your goal (for this year). The Key Results are what get you to the objective. They should be measurable. In the Check-in column, you can track what happened this month in regard to a specific Key Result. What went well, what went badly, and what are the roadblocks? Or just a comment. This allows you to review and make corrections to your workflows and planning.

Copy the following template and paste it into your templates folder, so you can use it on a monthly basis:

```
## Goal Tracking
🟩 = On Track
🟨 = At Risk
🟥 = Off Track

**Objective:** Increase the MRR by 30%
| Key Results | Target | Progress | Check-in |
| ----------- | ------ | -------- | -------- |
| Implement one growth initiative a month | 12 initatives | 🟩 2 | Reworked the paywall, and added A/B testing for trials |
| Ship one popular feature request a month | 12 features | 🟩 2 | Shipped dragging and checklists. |
| Decrease churn | 5% | 🟩 5.2% | Churn looks good |
```

### Tables for Habit Tracking

With a table, you can track habits on a weekly basis (for example). In the left column list all the days of this week and in the top row your habits. Then fill it out on a daily basis or once at the end of the week. This also serves as something you can review at the end of the week or when the next week begins:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6407709e9c8683055bad3881/file-KKzQsdwHkC.png)

Use a template that you can insert every new week. Either use the "Insert Template" button in the middle of an empty weekly note or type "/insert template", and select the habit tracking template in the line where you want to create the habit tracker table from your template. Here's a template to get you started. Copy it and paste it into a new template that you can create in the sidebar under "Templates":

```
---
title: 👟 Weekly Habit Tracker
type: empty-note 
documentation: https://help.noteplan.co/article/136-templates
---

## Habits
| **Day/Habit** | *Sleep 8h* | *Running* | *Read 30min* |
| ------------- | ---------- | --------- | ------------ |
| ~Monday~ | +&nbsp; | +&nbsp; | +&nbsp; |
| ~Tuesday~ | +&nbsp; | +&nbsp; | +&nbsp; |
| ~Wednesday~ | +&nbsp; | +&nbsp; | +&nbsp; |
| ~Thursday~ | +&nbsp; | +&nbsp; | +&nbsp; |
| ~Friday~ | +&nbsp; | +&nbsp; | +&nbsp; |
| ~Saturday~ | +&nbsp; | +&nbsp; | +&nbsp; |
| ~Sunday~ | +&nbsp; | +&nbsp; | +&nbsp; |
```

*Last updated: 2025-06-25*
