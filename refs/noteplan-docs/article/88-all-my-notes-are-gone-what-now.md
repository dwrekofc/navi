# All my notes are gone, what now?

Your notes are saved and loaded locally from your Mac or iOS device. This means there are ways you could accidentally delete everything. But there are ways to recover your data as well. If you just lost a few folders or notes, [check this article](131-i-lost-some-notes-folders-how-to-restore.md).

If you have the content of a note or it was overwritten by another device, you can [restore it](128-how-to-restore-a-note-to-an-earlier-version.md) from the original device that had the lost content.

In general, however, [make regular backups of your notes](32-how-to-make-a-backup-of-my-notes.md) for cases where it's not possible to recover your data because you have already emptied the Trash for example.

### 1. Restore recently deleted files

If you have accidentally deleted all your files, you can restore them in two different ways:

**A) Use "Recently Deleted..."**
On Mac, find this option under "File" and on iOS in NotePlan's preferences under the section "Files".

**B) Check your Trash folder**
NotePlan has an internal Trash folder and you can check your Mac's Trash folder as well. NotePlan moves deleted files there first before deleting them forever.

### 2. You just installed NotePlan

If you have added notes on one device (like your Mac) and just downloaded NotePlan on iOS (iPhone or iPad), it might take a little while until your notes arrive. It should sync your notes immediately after starting, but if it hangs, try to...

- Restart your device
- Restart NotePlan
- Update NotePlan on all devices, if an update is available
- Update your iOS or macOS
- See [here](24-how-to-fix-sync-problems.md) to troubleshoot for sync issues

### 3. Note extension was changed

If you have ever changed the note extension from "txt" to say "md", and you stopped seeing your daily notes inside NotePlan, it might be that you have changed this preference accidentally or the preference was deleted from your iCloud.

**To fix this problem**, open NotePlan's preferences and check under "Files" if the note extension is correct. NotePlan would also show you a dialog if a mismatch has been detected.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60d2088761c60c534bd6848a/file-KGt8ZZwf7V.png)

### 4. "On My Mac/iPhone/iPad" folder was deleted

It looks like there is nothing in this folder, but if you delete it, all your notes will be deleted too (locally).

This folder is pointing to the real folder, where all the data is stored for NotePlan. If you have deleted it, you can head over to the Trash and restore it.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60d2090c8556b07a28848650/file-5LvbdJpnOp.png)

**To restore it**, look for the folder `"co.noteplan.NotePlan3"`. It won't have the same name as in the one you have deleted. Then right-click and "Put Back". If it says the folder already exists, overwrite it. If you have NotePlan still running or restarted it, the folder was probably automatically recreated without notes in it.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60d209f400fd0d7c253f88b6/file-RELGRgOOcF.png)

### 5. iCloud or iCloud Drive was turned off

Even if you are syncing using CloudKit, it needs to have iCloud Drive enabled. If you have recently turned it off, it might have removed your notes automatically too. Turn it back on, and if you still can't see your notes, check inside your user folder for iCloud Drive archives:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60d20cbe05ff892e6bc29b96/file-FDpNtqjJKp.png)

They might contain NotePlan data, especially if you have used iCloud Drive for syncing.

### 6. Sync method switched without copying files

If you have switched the sync method from CloudKit to iCloud Drive or vice versa but haven't copied over your notes, you won't see them in NotePlan anymore.

Switching from iCloud Drive to CloudKit, NotePlan normally asks you if it should copy them automatically. If you missed that or you switched from CloudKit to iCloud Drive, you can simply copy them manually. See here [how to find your notes folder](31-where-are-my-notes-saved.md).

### 7. NotePlan removed on Mac with an "uninstaller app"

If you have used "CleanMyMac X" or other uninstaller apps which can remove apps for you, it most probably has also deleted your notes folder. In case you still want to keep your notes, because you just wanted to delete a beta version, for example, try to recover it directly from the uninstaller again.

Alternatively, you can check your Mac's Trash to see if it was moved there. If you see a folder named `"co.noteplan.NotePlan3"`, that's most probably the deleted folder. Right-click and click on "Put Back", if possible. If not, you need to manually move it back to `"~/Library/Containers/"`. Copy & paste this path into Finder -> `"Go"` -> `"Go to folder..."`. Then move NotePlan's folder back.

### 8. NotePlan deleted on iOS

If you have deleted NotePlan on iOS and are using CloudKit for syncing, iOS will also delete the locally saved files, including the notes. After reinstalling it, it should re-download your notes from CloudKit.

### 9. iCloud (Drive) storage was deleted

If you tried to make space in your iCloud Drive, or iCloud in general, you might have wiped out NotePlan data as well accidentally. To recover from this, you can re-upload your notes locally again using the advanced sync preferences in NotePlan.

If your notes were also deleted locally (means you see nothing in NotePlan), then check your Trash, if you can still find your notes there. If you see a folder named `"co.noteplan.NotePlan3"`, restore it using right-click "Put Back".

*Last updated: 2023-01-27*
