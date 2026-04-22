# Why is the sync slow [iCloud Drive]?

You can sync your notes in two main ways using NotePlan: iCloud Drive and CloudKit. We recommend CloudKit (it's a database by Apple, but your notes are still stored locally on your devices and we have no access to your data). If you are using iCloud Drive, the sync can be delayed.

NotePlan has no control over when the sync triggers, it can only save, load, and request sync from iCloud Drive. In the end, iCloud Drive runs a separate process that syncs your data at some point in time. You can speed up this process by opening the Finder or Files app and navigating to your iCloud Drive folder. There, you can also see the state of the sync process.

If you want a more reliable and faster sync, we recommend switching to CloudKit:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63da8a1d26cb8547dcb0e214/file-2cq3nWxykE.png)

*Last updated: 2023-02-01*
