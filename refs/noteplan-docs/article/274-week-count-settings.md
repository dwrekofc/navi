# Week Count Settings

The Week Calculation setting controls how week numbers are calculated in NotePlan. This affects week numbers in the calendar view and weekly note filenames.

### The Three Options

1. **ISO (Legacy)**
- Original NotePlan behavior
- Uses ISO 8601 with a configurable first day of week
- May differ from system calendars when using a non-Monday first day
2. **System**
- Uses your device's locale calendar
- Matches Apple Calendar week numbers
- Respects your "First day of week" preference
3. **ISO (Standard)**
- Pure ISO 8601 standard
- If your first day of week is not Monday, NotePlan corrects the week number automatically
- Uses the Thursday rule (week 1 contains the first Thursday of the year)

### How to Change the Setting

**On iOS:**

1. Open NotePlan
2. Tap on "Settings" in the left sidebar
3. Tap "Week Calculation"
4. Select your preferred option (a checkmark indicates the current selection)
5. The setting saves automatically

**On macOS:**

1. Open NotePlan
2. Go to **NotePlan > Preferences** (or press Cmd + ,)
3. Click the "Calendars" tab
4. Use the "Week Calculation" popup menu
5. Select your preferred option

### Important Notes

- Changing this setting may affect existing weekly note filenames if they were created with a different calculation method
- After changing the setting, you may see a "Migrate Weekly Notes" option to rename existing weekly notes to match the new calculation
- The setting syncs across your devices via iCloud (restart NotePlan if you don't see the change)
- Week numbers in the calendar view update immediately after changing the setting

### When to Use Each Option

- **Use ISO (Legacy)** if you want to maintain compatibility with existing weekly notes created in older versions of NotePlan
- **Use System** if you want week numbers to match Apple Calendar
- **Use ISO (Standard)** if you need strict ISO 8601 compliance for international standards or business use, matches Google Calendar

*Last updated: 2026-01-02*
