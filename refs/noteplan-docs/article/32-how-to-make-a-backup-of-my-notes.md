# How to make and restore a backup of my notes?

## Backup using CloudKit

If you are syncing with CloudKit, all your notes are uploaded to the CloudKit database (operated by Apple), which can serve as a backup if your Mac or iOS devices stop working. Your notes will be automatically re-downloaded once you run NotePlan on a new device.

NotePlan also saves versions of your notes in a local database on your device (different on each device). The versions will be gradually deleted the older they get. If you lost the content of a note, because it was overwritten by another device [you can restore this version](128-how-to-restore-a-note-to-an-earlier-version.md) on the original device that had this version of your note.

However, it is recommended that you also make manual backups.

### On Mac

You can use NotePlan to create a backup in the menubar under "File" -> "Create Full Backup":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61ec57f7d86136157d99c8db/file-uIlR7VMXYi.png)Or follow the manual process below to have full control over your backups:

1. [Reveal your notes in Finder](31-where-are-my-notes-saved.md) (click on "Show in Finder" in the note menu top right).![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611bfc58f886c9486f8d9b40/file-5do5bU4l5c.png)
2. Copy & paste NotePlan's folder `"co.noteplan.NotePlan3"` somewhere safe like your Desktop or Documents folder.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611bfca721ef206e5592c55f/file-1oDiEWwEfF.png)

### On iOS

Your notes are stored in Files app > On My iPhone/iPad > NotePlan. Here too, you can copy the full folder somewhere else on your device or cloud location.

1. Open the Files app ("Dateien in German"), locate NotePlan in "On My iPhone/iPad"
2. Long-press or right-click on "NotePlan" to reveal the context menu
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611bfd48b55c2b04bf6df35c/file-VQqUhz4GGS.jpg)
3. Tap on "Copy" or tap on "Compress"
1. If you have compressed the folder, either keep the zip file here or move it to another folder where you store your backups
2. If you hit "Copy", navigate to another folder, then long-press into empty space until the black context menu opens and select "Paste"

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611bfe82b55c2b04bf6df367/file-7EI4V7nxma.jpg)

###

## Backup using iCloud Drive

If you are syncing with iCloud Drive, your notes should be automatically backed up by TimeMachine unless you have turned it off. Even without your external backup disc, it creates local snapshots of the last 24h, read more here: [https://support.apple.com/en-us/HT204015](https://support.apple.com/en-us/HT204015).

You can revert a note to a previous version using TimeMachine by opening it with TextEdit, then click in the menu bar under "File" -> "Revert to". [Read here](31-where-are-my-notes-saved.md) to learn how to find your notes in Finder.

To make a full backup (so you can restore all at once), you can make a copy of your NotePlan iCloud Drive folder, similar to how you make backups using CloudKit explained before. Just the folder is different.

## Restore a backup

1. [Reveal your NotePlan's folder in Finder](31-where-are-my-notes-saved.md) (or Files app)
2. Close NotePlan, in case it's still running
3. Delete anything inside your NotePlans folder
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611bffeeb37d837a3d0e49e0/file-e76iK6ZxZA.png)
4. Copy the contents of your backup ("Calendar", "Notes", "Plugins", "Themes", etc.)
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611c004eb55c2b04bf6df373/file-wuU3qkUqhm.png)And paste the folders into NotePlan's folder.![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611c0087b55c2b04bf6df375/file-dmF7GnDqn1.png)
5. Launch NotePlan
6. If you have changed notes before restoring the backup and are using CloudKit:
1. you should close NotePlan on all devices (except the one from where you want to upload)
2. re-upload your local notes (see advanced sync preferences) to the cloud to make sure what you have locally is also reflected online
3. re-launch NotePlan on other devices (so they can pick up the backup files).
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611c011cf886c9486f8d9b64/file-8dakAkGq7A.png)

*Last updated: 2023-09-04*
