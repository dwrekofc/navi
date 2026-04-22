# How sync conflicts are detected

NotePlan tries to keep your files in sync by downloading changes to notes from the cloud database as soon as possible. However, when you have a bad internet connection, the download might be delayed and you could end up having a stale version of a note temporarily.

If you change a stale note before NotePlan had a chance to download the updated version, there will be two conflicting versions. NotePlan will warn you and give you a chance to select a version:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/608825d58af76a714bfdb738/file-4k167S7JwN.png)While you see this dialog, you can copy & paste the text somewhere safe in case merging the two notes is more complicated. Otherwise, just select a version by clicking on "Choose This". Click on "1/2" on the right to switch between the version.

**Under the hood,** NotePlan checks first if there is an update in the cloud before the stale and locally changed note is being uploaded. If it finds something with a file modification date newer than what is cached locally, it will trigger the warning. Essential for this mechanism is the file modification dates. Don't change them manually or by running a script or similar. They are essential to keep things in sync. Additionally, a hidden `.cache` file is saved in each major folder (Calendar, Notes, Themes,...) which saves the latest download/upload state.

*Last updated: 2021-05-09*
