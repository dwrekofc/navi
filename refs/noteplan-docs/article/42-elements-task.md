# Elements of a Task

Original: [Elements of a task in Noteplan 3](https://write.as/rhubarb/elements-of-a-task-v4) by Rhubarb

---

In NotePlan a **task** is a line of plain text in a Markdown file, that may look like this:

```
* Buy groceries for Alex
```

Depending on user preferences, containing notes, task states and previous interactions, a task could also look like this:

```
- [ ] Buy #groceries for @alex >2021-04-24
- [>] Buy #groceries for @alex >2021-04-24
- [x] Buy #groceries for @alex <2021-04-23 @done(2021-04-24)
```

What are the elements of a task in Noteplan?

## Two kinds of notes that can contain the task

|  |  |
| --- | --- |
| YYYYMMDD.md | day notes, daily notes |
| foobar.md | regular notes, general notes |

``    ``
## Five task states

|  |  |
| --- | --- |
| - | inactive, non-task |
| - [ ] | active, open, undone |
| - [x] | completed, done |
| - [-] | canceled, not to be done |
| - [>] | rescheduled, postponed |

``    ``    ``    ``    `` **
## Appended date and time

|  |  |  |
| --- | --- | --- |
| >YYYY-MM-DD | date tag* | only regular notes |
| @done(YYYY-MM-DD) | date stamp | only manual edit |
| @done(YYYY-MM-DD hh:mm) | time stamp |  |

``  **   ``  **   ``
*: do not confuse with link (see below)

## Link to notes

``  **   ``  **   ``
**   ``
|  |  |  |
| --- | --- | --- |
| [[YYYY-MM-DD]] | link to day note | only manual edit |
| [[foobar]] | link to a regular note | only manual edit |
| >YYYY-MM-DD | link* to day note with target copy of a rescheduled task | only day note |
| <YYYY-MM-DD | link to day note with source copy of a rescheduled task | only day note |

**
*: do not confuse with date tag (see above)

## Tags

|  |  |
| --- | --- |
| #foo | hashtags |
| @bar | mentions |
| >YYYY-MM-DD | date tag* |

``    ``    ``
*: only in regular notes (see above)

## Configurable syntax

Recognize `-`, `*`, `1.` as active tasks instead of non-tasks. Use `*` as default instead of `-` to render task states. A combination of both could create these task states:

|  |  |
| --- | --- |
| - | inactive, non-task |
| * | active, open, undone |
| * [x] | completed, done |
| * [-] | canceled, not to be done |
| * [>] | rescheduled, postponed |

``    ``    ``    ``    `` **

*Last updated: 2021-04-26*
