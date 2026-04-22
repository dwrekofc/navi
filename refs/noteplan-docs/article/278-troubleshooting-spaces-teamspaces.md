# Troubleshooting Spaces (Teamspaces)

## Space not appearing or missing after reinstall

Spaces sync separately from your private notes using a different database. If a Space doesn't appear after reinstalling NotePlan or setting up a new device, try the following steps in order.

### Step 1: Restart NotePlan

Quit NotePlan completely and reopen it. Give it a minute to connect and download your Spaces data.

### Step 2: Log out and log back in to Spaces

Open Settings > Spaces and log out. Then log back in with the same email address you originally used to create or join the Space. Spaces use their own email-based login, separate from your Apple ID.

### Step 3: Check the web version

Log in to [app.noteplan.co](https://app.noteplan.co) with your Apple ID. Then click the settings button (top left) and log in to Spaces with your Spaces email. If the Space appears in the web version, it's still on the server and should sync back to your devices.

### Step 4: Repair the Spaces database

In Settings > Spaces, click "Advanced" (bottom left) to find the repair option. This will attempt to fix any database inconsistencies without deleting your local data.

### Step 5: Delete and re-download the Spaces database

If the repair didn't help, you can force a full re-download:

1. Close NotePlan.
2. Open NotePlan's sync folder using "Show in Finder" (open any private note, then open the menu top right, then "Show in Finder").
3. Open the "Caches" folder.
4. Delete (or move to a backup location) these files: `teamspaces.db`, `teamspaces.db-shm`, and `teamspaces.db-wal`.
5. Reopen NotePlan and wait for it to re-download your Spaces from the server.

See also: [How to Reload my Local Teamspaces Database?](263-how-to-reload-my-local-teamspaces-database.md)

## Spaces notes not syncing between devices

If your Spaces notes appear on one device but not another, or edits aren't showing up:

- Make sure you're logged into Spaces with the same email on all devices.
- Restart NotePlan on the affected device.
- Check your internet connection.
- Try the repair or re-download steps above (Steps 4 and 5).

## Database error ("database disk image is malformed")

If you see this error related to Spaces, follow Step 5 above to delete and re-download the Spaces database. See also: [Repair Local Sync Database](234-repair-local-sync-database.md)

## Still not working?

If none of the above steps resolve the issue:

1. **Create a database log file:** Go to Settings > Spaces > Advanced and click "Create Database Log File". See: [How to Create a Teamspaces Database Log File](262-how-to-create-a-teamspaces-database-log-file.md)
2. **Enable logging:** On macOS, go to the menu bar > Help > Enable Logging. Wait a few seconds for NotePlan to capture some activity. See: [How to enable logging?](57-how-to-enable-logging.md)
3. **Send both files to us** at [[email protected]](cdn-cgi/l/email-protection.md) and describe what you're seeing.

*Last updated: 2026-03-05*
