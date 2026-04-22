# Advanced Search

NotePlan allows you to search for notes or for a specific line (like a task, a bullet, etc.) inside a note:

1. Search for for a **note by title**: Command Bar (CMD+J) or tap on the magnifier glass icon on iOS
2. Search and **filter notes**: Folder Views. Click on any folder or on "Notes" in the sidebar, then configure a filter to find specific notes.
3. Search for **note content**: Open "Search" in the left sidebar or CMD+Shift+F to find tasks, bullets, or any lines inside your notes.

**➡️ View the full video [here](https://youtu.be/P2HFpJtOcb8).**

## Advanced Search Syntax

The content search (open "Search" or "Filters" in the sidebar), and the Folder View search supports the following syntax.

*Note: Boolean OR and grouping is supported from v3.18.1*

- **Exact matches (whole words)**: surround a word with quotes. Example: `"sun"` returns lines where *sun* appears as a word, not as part of *sunlight*.
- **Exclusions**: prepend a hyphen to exclude. Example: `#pending -#waiting` returns results with *#pending* that do *not* also contain *#waiting*.
- **Implicit AND**: a space means AND. Example: `project report` returns lines that contain *both* *project* and *report*.
- **Boolean OR**: use `OR` (uppercase) to match *either* side. Examples:
- `meeting OR meetup` — matches if either word is present.
- `#work OR #personal` — matches notes/lines containing either tag.
- **Grouping with parentheses**: use parentheses to control precedence and combine expressions.
- `meeting (work OR meetup)` — must include "meeting" and either "work" or "meetup".
- `(urgent OR high) #todo` — includes either "urgent" or "high" and the *#todo* tag.
- `(#work OR #personal) (review OR recap)` — at least one tag group and one keyword group must match.
- **Grouped negation**:
- `meeting -(work meetup)` — includes "meeting" but excludes lines that contain *both* "work" and "meetup" together.
- `(draft OR proposal) -#archived` — includes either "draft" or "proposal" but excludes the *#archived* tag.
- **Quoted phrases inside groups**:
- `("design review" OR "code review") #todo` — matches either phrase plus *#todo*.
- **Nesting**: you can nest groups for complex logic.
- `(projectA (design OR testing)) OR (projectB (spec OR "kick off")) `

*Tips*:

- Combine spaces (AND), `OR`, `-` (NOT), and parentheses to craft precise queries.
- Use uppercase `OR` for clarity when combining alternatives (as shown in examples).

On Mac you can additionally decide what sources NotePlan should search: Calendar Notes, Events, Reminders or only Regular Notes (disable everything). You can find a settings drop down right of the search field on Mac to configure this.

## Search Operators

Use search operators at the beginning of your search to quickly filter results. Format: `operator:value ... search text`

*Note: Supported from v3.18.1*

- **Source operators**:
- `source:calendar` - Calendar notes only
- `source:notes` - Regular notes only
- `source:events` - Events only
- `source:reminders` - Dated reminders grouped by date
- `source:list-reminders` - All reminders grouped by list (dated and undated)
- `source:dated-notes` - Dated notes only
- `source:notes,events` - Multiple sources (comma-separated)
- **Task status operators**:
- `is:open` - Open tasks
- `is:done` - Completed tasks
- `is:scheduled` - Scheduled tasks
- `is:canceled` or `is:cancelled` - Canceled tasks
- `is:not-task` - Lines that are not tasks (this returns only results if you also supply a search query)
- **Checklist status operators**:
- `is:checklist` - Open checklist items
- `is:checklist-done` - Completed checklist items
- `is:checklist-scheduled` - Scheduled checklist items
- `is:checklist-cancelled` - Cancelled checklist items
- **Date operators**:
- `date:today` - Today's items
- `date:yesterday` - Yesterday's items
- `date:tomorrow` - Tomorrow's items
- `date:past` - Past items
- `date:future` - Future items
- `date:past-and-today` - Past items including today
- `date:this-week` - This week
- `date:last-week` - Last week
- `date:next-week` - Next week
- `date:this-month` - This month
- `date:last-month` - Last month
- `date:next-month` - Next month
- `date:this-year` - This year
- `date:last-year` - Last year
- `date:next-year` - Next year
- `date:30days` - Rolling 30 days
- `date:all` - All time (default if nothing is defined)
- **Custom ISO dates**:
- `date:2025-08-25` - Specific day (YYYY-MM-DD)
- `date:2025-W35` - Specific week (YYYY-WNN)
- `date:2025-08` - Specific month (YYYY-MM)
- `date:2025-Q3` - Specific quarter (YYYY-QN)
- `date:2025` - Specific year (YYYY)
- **Date ranges (i)**:
- `date:2025-08-01-2025-08-30` - Day range (YYYY-MM-DD-YYYY-MM-DD)
- `date:2025-W01-2025-W52` - Week range (YYYY-WNN-YYYY-WNN)
- `date:2025-06-2025-07` - Month range (YYYY-MM-YYYY-MM)
- `date:2025-Q1-2025-Q4` - Quarter range (YYYY-QN-YYYY-QN)
- `date:2024-2025` - Year range (YYYY-YYYY)
- **Examples**:
- `date:2025-08-25 meeting` - Meetings on August 25, 2025
- `date:2025-W35 review` - Reviews in week 35 of 2025
- `date:2025-08-01-2025-08-30 vacation` - Vacation items for August 2025
- `date:2025-Q1-2025-Q4 project` - Project items for all of 2025
- `source:calendar date:2025-08-01-2025-08-30 is:open meeting` - Open calendar meetings in August 2025
- **Location operators**:
- `path:Projects/Work` - Search in specific folder path
- `path:"10 - Projects/Marketing"` - Use quotes for folders with spaces
- You don't need to include "Notes" into the path.
- Paths are case-sensitive
- `heading:TODO` - Search under specific heading
- `heading:"Project Goals"` - Use quotes for headings with spaces
- **Sort operators**:
- `sort:asc` - Sort results by date from oldest to newest (past **→ **future)
- `sort:desc` - Sort results by date from newest to oldest (future **→ **past)
- **View options**:
- `show:timeblocked` - Show timeblocked events
- `hide:past-events` - Hide past events
- `hide:archive` - Hide results from archived notes (available from v3.18.2)
- `hide:teamspaces` - Hide results from teamspace (calendar) notes (available from v3.18.2)
- **Combined operators**:
- `source:calendar is:open date:this-week meeting` - Combine multiple filters
- `heading:"Weekly Review" is:open tasks` - Find open tasks under a specific heading with spaces
- Search operators can be combined with advanced syntax: `source:notes is:open (urgent OR high) -#archived`

*Notes:*

- Search operators require spaces to separate multiple operators
- Do not use spaces after colons (e.g., use `source:calendar` not `source: calendar`)
- For values with spaces (like heading names), wrap them in quotes: `heading:"My Project Name"`
- You can escape operators with a backslash if you want to search for them literally, e.g. `\date:this-month`

*Last updated: 2025-09-11*
