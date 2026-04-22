# How to fix sync problems?

NotePlan synchronizes your notes by default using CloudKit. You can change it to iCloud Drive in the preferences. If you are using Dropbox, please change it to iCloud Drive or CloudKit. Dropbox is not working completely and is not recommended to use.

If you have the content of a note or it was overwritten by another device, you can [restore it](128-how-to-restore-a-note-to-an-earlier-version.md) from the original device that had the lost content.

If the sync is not working, please try the following checklist:

### 1. Reboot & Internet Connection

- Fully reboot your Mac and iOS devices to make sure nothing is stuck in a temporary issue.
- Sign out with your Apple ID and sign in again.
- Make sure you’re properly connected to the Internet.
- If syncing is slow while you’re on a mobile device, try connecting to Wi-Fi and charging your device. iCloud/CloudKit syncing is optimized for saving power and runs slower in Low Power Mode.
- If you’re on a cellular network (3G, 4G, LTE, E), please verify that the options “Cellular Data”, “NotePlan” and “iCloud Drive” are activated in iOS Settings › “Cellular”.
- In very rare cases, iCloud might be slow or even inaccessible due to server issues. You can look up its status anytime on Apple's [system status page](http://www.apple.com/support/systemstatus/).

### 2. Check NotePlan's Preferences

- Make sure you are using the same **sync method** on all devices in the preferences under "sync": iCloud Drive or CloudKit.
- If NotePlan can find files in your iCloud Drive, it will choose this method automatically. This can happen after a reinstall which wipes out locally saved preferences. If you are using CloudKit on other devices, they won't sync with this device.
- If your daily notes don’t seem to sync at all, check if you are using the same **file extension** on all devices and adjust the preferences if needed (under “Files”). See [All my notes are gone](88-all-my-notes-are-gone-what-now.md) for more details on this.

### 3. Check System Preferences

- Check if **iCloud and iCloud Drive are enabled** on all your devices and you are logged in with the same Apple ID into your iCloud account.
- On Mac, iCloud Drive must be enabled in System Preferences › “Apple ID”.
- On iOS, please ensure “iCloud Drive” and “NotePlan” are activated in iOS’ Settings › **your account name** › “iCloud”.
- [Check these steps](258-how-to-resolve-the-sync-error-you-need-to-enable-icloud-drive-cloudkit-is-part-of-it.md) if you get an error saying that iCloud is not activated.
- If you are using iCloud Drive syncing, [turn off "Optimize Mac Storage"](89-improve-icloud-drive-sync.md).
- Please verify you don‘t exceed your [iCloud storage limit](https://support.apple.com/HT204247). Otherwise, NotePlan is not able to sync your notes across your devices. Both, CloudKit and iCloud Drive are relying on your iCloud storage.
- Please ensure that no firewalls such as LittleSnitch or in some cases AdBlockers are blocking iCloud services.
- Sometimes macOS / iOS is prompting you to log in again into iCloud or finish some setup steps in your System Preferences.

### 4. Check what is being Synced

- Make sure your devices are running the same versions of NotePlan.
- If you are syncing a lot of notes at once, please be patient. Sometimes it just takes a while to sync everything (especially when uploading) or your upload might be temporarily rate-limited by Apple's servers. This usually resolves in 24 hours.
- Make sure you are not syncing an excessive amount of files. You can [open the sync folder](31-where-are-my-notes-saved.md) and check if files got in which shouldn't be there and could block the regular sync.
- Very large non-textfile files, like huge PDF files, images, videos, etc.
- Folders with a large number of files (plain text files, like in Markdown format are ok).
- Plugins with a lot of library files.

### 5. Reset or Switch Sync Method

- **Reset everything** after [creating a backup](32-how-to-make-a-backup-of-my-notes.md) by deleting the local CloudKit folder and reinstalling NotePlan. [See here how to find the folder.](31-where-are-my-notes-saved.md)
- If CloudKit is not working for you, please use iCloud Drive. You can change it in NotePlan's preferences under "Sync".

*Last updated: 2025-03-13*
