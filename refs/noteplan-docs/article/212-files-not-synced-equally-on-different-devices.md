# Notes not synced equally or having a mismatch on different devices or web version

Sometimes one device might show a slightly different set of files than the other. You might see an old folder structure that you have previously renamed on one device. But it hasn't been synced to the other.

This can happen when there is some sync problem or issues with moving notes and renaming folders.
Before you continue, make sure it's not a different problem:

- Are you using the **same Apple ID** across devices and the web version (iCloud login in the system settings/web login)?
- Did you give NotePlan **a chance to upload (or download)** the changed notes (check internet connection) and you don't see any errors?

To fix this problem, we recommend re-uploading all your files to the database from the device that has the up-to-date file set. Before you do this, however, always create a backup.

## iOS

Create a backup on iOS by navigating to the notes in Files app > "On my iPhone/iPad" > NotePlan and copy the folder or zip

On iOS you can re-upload all notes by:

1. Navigating to NotePlan's settings (open the left sidebar, scroll down and tap on "Settings).
2. Inside the settings scroll down to the "Sync" section.
3. Tap on "CloudKit Advanced Options".
4. Tap on "Delete Cloud and Re-Upload"

## macOS

The easiest way on Mac is through File > Create Full Backup or open the sync folder and make a copy anywhere.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/64e8ceab42e1f64bf802668d/file-DgbvL58IMo.png)
To re-upload your files, open the Sync preferences, and click on "Advanced":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/64e8cee36ca77422d096481b/file-wjQdkvvWvE.png)There, click on "Delete Cloud and Re-Upload". The settings are similar on iOS. This will reset the CloudKit database effectively. The other devices should notice this and download the changes.

They won't delete anything locally though, so you might need to clean up duplicated folders manually. Or if you want to start from a clean slate, delete all the files on other devices (on Mac you need to delete the complete [sync folder](31-where-are-my-notes-saved.md), on iOS you just need to delete and re-install the app). Make sure NotePlan is quit when you delete things. After restarting NotePlan, it will re-download the files from the Cloud.

Direct path for the AppStore version: `~/Library/Containers/co.noteplan.NotePlan3/Data/Library/Application Support/co.noteplan.NotePlan3`

Direct path for the Setapp version: `~/Library/Containers/co.noteplan.NotePlan-setapp/Data/Library/Application\ Support/co.noteplan.NotePlan-setapp`

To check the file counts on different devices, you can use the "Note Statistics" plugin with the "/note stats" command:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/64e8cde6e82ed15ede51f2bb/file-r9iIraSwT3.png)

*Last updated: 2025-01-16*
