# How to download plugins on iOS/iPadOS

Note: With v3.4 you can install plugins on iOS too now, [read here more](127-install-plugins-on-ios.md).

---

Currently (v3.0.26) it is not possible to download plugins directly in the iOS / iPadOS version of NotePlan. This might change in future releases.

You can download plugins on Mac (in the preferences > Plugins) and they will get synced automatically to your iPhone or iPad. This is the easiest way, if you have a Mac available. If you have no Mac available, see the workaround below.

### Download Plugins on iOS

Plugins are stored in the same location as your notes, in a sub-folder called “Plugins”. You can open this folder using the Files app on your iPhone or iPad.

You can download individual plugins from the official plugin repository: [https://github.com/NotePlan/plugins/releases](https://github.com/NotePlan/plugins/releases). A plugin consists usually of one configuration file and one script file.

Here are the steps:

1. Go to the [repository](https://github.com/NotePlan/plugins/releases) and find the plugin you want to download.
2. Expand the list of files under “Assets” and download “plugin.json” and “script.js” (the name of the script file might be different). Usually, you would need at least those two files, but it might contain more scripts. You don’t need to download the “source code”.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61228961b55c2b04bf6e088b/file-bwY7Odd4gP.png)
3. Go into the Files app and find the two (or more) downloaded files under “Downloads” and select them.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61228a25f886c9486f8dafcd/file-75tKitcU9a.png)
4. Tap on “more” at the bottom right (on iPad) to create a new folder with the two items in there (you can also create the folder first and then move the files). Use the name of the plugin as the folder name (it’s not important how the folder is named): ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61228a6721ef206e5592d980/file-H22Go0S1b9.png)
5. Now move the folder into NotePlan’s plugins location. You can find it in the Files app under “On my iPhone/iPad” > NotePlan > Plugins, if you are using CloudKit to sync. If you are using iCloud Drive to sync, you will find NotePlan’s folder in your iCloud Drive folders instead.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61228b62b37d837a3d0e5dfd/file-jZN7PBWdrk.png)
6. Launch NotePlan and open the command bar (CMD+J or tap on the magnifier glass icon bottom right) or start typing into the text anywhere with “/“. You should see the commands from the downloaded plugin now and be able to execute them.

*Last updated: 2022-01-29*
