# Dates in Templates

You can make NotePlan print the current date using the following tag in your templates:

```
<%- date.now("Do MMMM YYYY") %>
```

By changing the content in the brackets ( `"Do MMMM YYYY"`), you can change the format of the date. This particular format will print the following into your note:

```
6th June 2022
```

Besides changing the format you can also calculate a different date by adding or subtracting days, weeks, months, years, etc. from the current date to get a different one by adding another variable (number):

```
<%- date.now("Do MMMM YYYY", -2) %>
```

The above tag will subtract 2 days from now. So it will print the day before yesterday:

```
4th June 2022
```

## Date Formats

*(Templating uses "[momentjs](https://momentjscom.readthedocs.io/en/latest/moment/04-displaying/01-format/)" under the hood, the following table is a copy from the documentation.)*

You can assemble a format from the following options below. Insert them into the brackets of the date tag: `<%- date.now("Do MMMM YYYY") %>`. For example, add the current time and day of week: `ddd Do MMM HH:mm`. Which will print: `Mon 6th Jun 11:18`.

****                         ****          ****               ****               ****                         ****     ****     ****               ****               ****
****[](https://tc39.es/ecma262/#sec-expanded-years)
****   ****     ****
****
****
****   ****          ****          ****          ****                              ****          ****          ****                    ****
|  | Token | Output |
| --- | --- | --- |
| Month | M | 1 2 ... 11 12 |
|  | Mo | 1st 2nd ... 11th 12th |
|  | MM | 01 02 ... 11 12 |
|  | MMM | Jan Feb ... Nov Dec |
|  | MMMM | January February ... November December |
| Quarter | Q | 1 2 3 4 |
|  | Qo | 1st 2nd 3rd 4th |
| Day of Month | D | 1 2 ... 30 31 |
|  | Do | 1st 2nd ... 30th 31st |
|  | DD | 01 02 ... 30 31 |
| Day of Year | DDD | 1 2 ... 364 365 |
|  | DDDo | 1st 2nd ... 364th 365th |
|  | DDDD | 001 002 ... 364 365 |
| Day of Week | d | 0 1 ... 5 6 |
|  | do | 0th 1st ... 5th 6th |
|  | dd | Su Mo ... Fr Sa |
|  | ddd | Sun Mon ... Fri Sat |
|  | dddd | Sunday Monday ... Friday Saturday |
| Day of Week (Locale) | e | 0 1 ... 5 6 |
| Day of Week (ISO) | E | 1 2 ... 6 7 |
| Week of Year | w | 1 2 ... 52 53 |
|  | wo | 1st 2nd ... 52nd 53rd |
|  | ww | 01 02 ... 52 53 |
| Week of Year (ISO) | W | 1 2 ... 52 53 |
|  | Wo | 1st 2nd ... 52nd 53rd |
|  | WW | 01 02 ... 52 53 |
| Year | YY | 70 71 ... 29 30 |
|  | YYYY | 1970 1971 ... 2029 2030 |
|  | YYYYYY | -001970 -001971 ... +001907 +001971 Note: Expanded Years (Covering the full time value range of approximately 273,790 years forward or backward from 01 January, 1970) |
|  | Y | 1970 1971 ... 9999 +10000 +10001 Note: This complies with the ISO 8601 standard for dates past the year 9999 |
| Era Year | y | 1 2 ... 2020 ... |
| Era | N, NN, NNN | BC AD Note: Abbr era name |
|  | NNNN | Before Christ, Anno Domini Note: Full era name |
|  | NNNNN | BC AD Note: Narrow era name |
| Week Year | gg | 70 71 ... 29 30 |
|  | gggg | 1970 1971 ... 2029 2030 |
| Week Year (ISO) | GG | 70 71 ... 29 30 |
|  | GGGG | 1970 1971 ... 2029 2030 |
| AM/PM | A | AM PM |
|  | a | am pm |
| Hour | H | 0 1 ... 22 23 |
|  | HH | 00 01 ... 22 23 |
|  | h | 1 2 ... 11 12 |
|  | hh | 01 02 ... 11 12 |
|  | k | 1 2 ... 23 24 |
|  | kk | 01 02 ... 23 24 |
| Minute | m | 0 1 ... 58 59 |
|  | mm | 00 01 ... 58 59 |
| Second | s | 0 1 ... 58 59 |
|  | ss | 00 01 ... 58 59 |
| Fractional Second | S | 0 1 ... 8 9 |
|  | SS | 00 01 ... 98 99 |
|  | SSS | 000 001 ... 998 999 |
|  | SSSS ... SSSSSSSSS | 000[0..] 001[0..] ... 998[0..] 999[0..] |
| Time Zone | Z | -07:00 -06:00 ... +06:00 +07:00 |
|  | ZZ | -0700 -0600 ... +0600 +0700 |
| Unix Timestamp | X | 1360013296 |
| Unix Millisecond Timestamp | x | 1360013296123 |

****     ****
## Date Calculation

By adding a number as a second variable to the date tag you can change the relative date, relative to today. If you just use a number you can add or subtract days:

```
<%- date.now("Do MMMM YYYY", -2) %>
```

If you want to add/subtract weeks ("1w"), months ("1M") or years ("1y") use the following:

```
In two weeks:
<%- date.now("Do MMMM YYYY", "2w") %>

In two months:
<%- date.now("Do MMMM YYYY", "2M") %>

In two years:
<%- date.now("Do MMMM YYYY", "2y") %>
```

## Get the date of the current daily note

There is no specific tag to get the date of the current daily note, but you can use the title of the currently opened note and format it to a proper date:

```
<%- date.format("Do MMMM YYYY", Editor.title) %>
```

## Date Tags

Besides `date.now(...)`, you can use the following additional date tags in your templates. Some tags support dates as input variables or return dates and they are normally formatted as "YYYY-MM-DD".

- **weekday**: `<%- date.weekday("Do MMMM YYYY", 2, '2021-12-15') %>` = `2021-12-17`
- Returns the closest weekday with an optional offset (here "2" = "in 2 days"). Weekends are skipped.
- If you don't use a date as the third variable, it uses the current date.
- **weekNumber**: `<%- date.weekNumber('2021-12-15') %>` = `50`
- Prints the current week number for a given date (formatted as "YYYY-MM-DD").
- Leave the text ('2021-12-15') empty to get today's week number.
- **dayNumber: **`<%- date.dayNumber('2021-12-15') %>` = `2021-12-17`
- Returns the number of the day for a given date ("YYYY-MM-DD") in the week where Sunday = 0, Monday = 1, Tuesday = 2, Wednesday = 3, Thursday = 4, Friday = 5, Saturday = 6.
- Leave the date field empty to get the day number of today.
- **isWeekday**: `: <%- date.isWeekday('2021-12-15')%>` = `true`
- Returns true if the date is a weekday and false if it's a weekend day.
- **isWeekend**: `<%- date.isWeekend('2021-12-15') %>` = `false`
- Returns true if the date is a weekend and false if it's a weekday.
- **weekOf**: `<%- date.weekOf('2021-12-01') %>` = `W48 (2021-11-28..2021-12-03)`
- Returns a formatted week number of the given date range.
- **startOfWeek**: `<%- date.startOfWeek('YYYY-MM-DD', '2021-12-01', 0) %>` = `2021-11-28`
- Returns the first day of the week = Sunday for a given date formatted as "YYYY-MM-DD" text.
- In the first variable you can change the input format, the second variable contains the date as text using that format and in the last variable you can change the first day of week which is by default Sunday = 0. If you want to use Monday, input `1` for the last variable.
- ** endOfWeek**: `<%- date.endOfWeek('YYYY-MM-DD', '2021-12-01', 0) %>` = `2021-12-04`
- Returns the last day of the week = Saturday for a given date formatted as "YYYY-MM-DD" text.
- In the first variable you can change the input format, the second variable contains the date as text using that format and in the last variable you can change the first day of week which is by default Sunday = 0. If you want to use Monday, input `1` for the last variable.
- **format**: `<%- date.format('Do MMMM YYYY', '2021-12-01') %>` = `1st December 2021`
- Format a given date text formatted as "YYYY-MM-DD" to another format.
- This is useful if you want to take for example the date of the current daily note using `Editor.title` as the second argument and convert it to a good looking date.

---

*Available either in the current version of the np.Templating plugin or in the next NotePlan v3.6 using the "Insert Template" button.*

- ** startOfMonth **: `<%- date.startOfMonth('YYYY-MM-DD', '2021-12-06') %>` = `2021-12-01`
- Returns the first day of the month for a given date formatted as "YYYY-MM-DD" text.
- Available in np.Templating v1.5
- ** daysInMonth **: `<%- date.daysInMonth( '2021-12-06') %>` = `31`
- Returns the amount of days in a month as number.
- ** endOfMonth **:`<%- date.endOfMonth('YYYY-MM-DD', '2021-12-06') %>` = `2021-12-31`
- Returns the last ay of a month for a given date formatted as "YYYY-MM-DD" text.
- ** daysBetween **: `<%- date.daysBetween('2021-12-03', '2021-12-24') %>` =`21`
- Returns the amount of dates between two given dates as number.
- ** add **: `<%- date.add('2022-01-07', '7', 'days', 'YYYY-MM-DD') %>` =`2022-01-14`
- Adds a number of days to a given date and returns the date formatted as "YYYY-MM-DD" text.
- ** businessAdd **: `<%- date.businessAdd(3, '2022-01-07', 'YYYY-MM-DD') %>` = `2022-01-12`
- Adds a number of business days to a given date and returns the date formatted as "YYYY-MM-DD" text. Weekends are skipped.
- ** subtract **: `<%- date.subtract('2022-01-12', '7', 'days', 'YYYY-MM-DD') %>` = `2022-01-05`
- Subtracts a number of days to a given date and returns the date formatted as "YYYY-MM-DD" text.
- ** businessSubtract **: `<%- date.businessSubtract(3, '2022-01-12', 'YYYY-MM-DD') %>` = `2022-01-07`
- Subtracts a number of business days to a given date and returns the date formatted as "YYYY-MM-DD" text. Weekends are skipped.
- ** nextBusinessDay **: `<%- date.nextBusinessDay('2022-01-12', 'YYYY-MM-DD') %>` = `2022-01-13`
- Returns the next business date relative to the given date formatted as "YYYY-MM-DD" text.
- ** previousBusinessDay **: `<%- date.previousBusinessDay('2022-01-10', 'YYYY-MM-DD') %>` = `2022-01-07`
- Returns the previous business date relative to the given date formatted as "YYYY-MM-DD" text.

---

### Contributions

Thanks to the plugin community, with special thanks to Mike (aka [@codedungeon](https://mobile.twitter.com/codedungeon)) for creating this templating engine and the plugins based on it! You can contribute with your ideas, reports, feedback or by working directly on plugins by visiting our [Discord](https://discord.gg/D4268MT) community or our [GitHub repository](https://github.com/NotePlan/plugins). This documentation is based on the [original](https://noteplan.co/templates/docs/templating-modules-date).

*Last updated: 2025-05-18*
