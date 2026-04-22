# How to access a note version directly from the versions database?

NotePlan stores versions of your note inside a local SQL database. Usually, you can restore a note using the menu button top right, then "View Revisions".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/639b2ce7f44c8a11914acaa2/file-Gccpnm4Lje.png)However, if you are facing issues with accessing or restoring a version through this menu like NotePlan is crashing, for example, you can still access your note from the database directly. The database file is saved locally on your Mac or iOS device.

These are the steps:

1. Locate the database file.
2. Open the database file.
3. Find the note.
4. Copy the note content.

The SQL query: `select data from versions where file_id = (select file_id from files where relative_path = "RELATIVE PATH");`

## 1. Locate the database file

The database file is in the "Caches" folder of NotePlans sync location. We will need the Terminal to go to this folder and locate the database.

1. Open the note you want to restore in NotePlan.
2. Open the Terminal window (search for "Terminal" in Spotlight)
3. Type "cd " (don't forget the space, cd means "change directory").
4. Get the path to your note by opening it in Finder (click on the menu button top right, then "Show in Finder").
5. Find the folder "Caches" and drag it into the Terminal window. The path will be pasted for you.
6. Hit enter and you should be inside the Caches folder using the Terminal.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/639b309bf44c8a11914acaa9/file-BGohpxdQ78.gif)

## 2. Open the database file

To open the database type "sqlite3 versions.db", then hit enter. Then you should be inside the database.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/639b302b2e586565571c2121/file-hVN7yh13qi.gif)

## 3. Find the note

To find the note we need a SQL command and the relative path (filename) of your note:

1. Paste the SQL command into your Terminal:
- `select data from versions where file_id = (select file_id from files where relative_path = "RELATIVE PATH");`
2. Get the relative path by right-clicking the note in the left sidebar and then "Copy Relative Path".
3. Replace `RELATIVE PATH` with the copied path.
4. Hit enter and you should see some versions of your note.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/639b31cf8527f140b63ed836/file-1fmUcy1Kdk.gif)

## 4. Copy the note content

With the above steps, you see all versions of your note, but you can modify the SQL query slightly to show you only the most recent versions and limit the output. This query shows you the 3 most recent versions:

`select data from versions where file_id = (select file_id from files where relative_path = "RELATIVE PATH") order by date desc limit 3;`

You can modify the number at the end to show you a different amount of versions. Find the one you want to keep and copy the text from the Terminal into NotePlan.

*Last updated: 2022-12-15*
