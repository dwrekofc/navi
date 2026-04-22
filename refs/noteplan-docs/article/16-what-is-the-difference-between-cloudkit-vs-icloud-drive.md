# What is the difference between CloudKit vs iCloud Drive?

NotePlan saves all your notes as plain markdown text files on your device. By default, it syncs your notes across your devices using CloudKit. You can choose to switch the sync to iCloud Drive. Both are storage and sync services created and managed by Apple with built-in encryption and security. We can't see any of your data, because nothing is saved on our servers.

In short, we recommend using **CloudKit**. Longer explanation below.

## CloudKit

CloudKit is a database with an interface for developers to interact with it. Using CloudKit, NotePlan can actively decide when to upload or download data to Apple's servers. This makes sync faster and more reliable. It also offers more features NotePlan will make use of in the future like sharing notes or accessing notes through a web version.

Your notes are saved in a folder locally on your device. You can open an existing note in NotePlan, then top right-click on the menu button and "Show in Finder" to open this folder.

## iCloud Drive

iCloud Drive doesn't give NotePlan control over the sync. Your notes are just saved into your iCloud Drive folder and the operating system decides when and what to upload or download. This might sometimes cause sync delays and you need to open the Finder (Mac) or Files app (iOS) to kick off the sync.

NotePlan is also not getting error messages from iCloud Drive and troubleshooting sync issues is harder.

*Last updated: 2023-02-01*
