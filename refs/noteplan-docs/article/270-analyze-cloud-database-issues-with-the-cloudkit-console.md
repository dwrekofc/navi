# Analyze Cloud Database Issues with the CloudKit Console

The CloudKit Console is available only on the Mac in the Settings > Sync > Advanced (below "CloudKit") > Open CloudKit Console.

It will load all notes from the cloud into a table and compare against the local database. At the top you would see if there are any duplicates or other inconsistencies.
See an example below. You can see a "Duplicates" label at the top and the "Is Duplicate", "Exists in local cache", "Exists as file" columns. Sort them ascending/descending to see if there are any issues.

From this dialog you can also delete individual notes or duplicates. In any case, make sure you have a backup of all your notes locally.

If you see that there are a lot of duplicates, the safest way to fix it is to "Delete Cloud and Re-Upload". It won't delete anything local, just rebuild the cloud database, so it's cleaned up. This will trigger a download on all other devices, so if you have a lot of notes, it might take some time to go through this until you can sync again in real-time.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/689b4c4f902fdd73a50571b6/file-yE3z3gfVm5.png)

*Last updated: 2025-08-12*
