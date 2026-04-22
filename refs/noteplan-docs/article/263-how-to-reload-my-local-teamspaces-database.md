# How to Reload my Local Teamspaces Database?

You can re-download all your Teamspaces notes by deleting the database file. NotePlan will download everything from the server.

Create a backup of

1.  Close NotePlan
2. Open NotePlan's sync folder using "Show in Finder" (open any private note, then open the menu top right, then "Show in Finder"):
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67f9672d9a992a1d064d8b24/file-zjZERFq1lw.png)
3. Open the folder "Caches", then delete (or cut and paste so you have a backup) teamspaces.db, teamspaces.db-shm, and teamspaces.db-wal
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67f9678c0c8551125243a18e/file-efTFyThqTp.png)
4. Open NotePlan and wait a few seconds for the download.

If this is not helping, you can [create a database log file](262-how-to-create-a-teamspaces-database-log-file.md) and write us.

*Last updated: 2025-04-11*
