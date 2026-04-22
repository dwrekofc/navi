# How to force sync?

Forcing NotePlan to sync "right now" because useful in situations where you want to [troubleshoot sync problems](24-how-to-fix-sync-problems.md). Or, if for some reason your sync stopped or is not frequent enough. Also, you might want to make sure the latest change is synced before you close your device.

## Using iCloud Drive Sync

Apple doesn't provide a way to force sync iCloud Drive by code, but you can still trigger manually with the following actions.

### On Mac

You can force iCloud Drive to sync by opening your iCloud Drive folder using Finder. Or click the top right of a note on the menu button, then on "Show in Finder". This will tell iCloud Drive it should sync now.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60d3622a05ff892e6bc2a21d/file-tZPbwHMrWy.png)

### On iOS

Open the Files app ("Dateien" in German) and navigate to your iCloud Drive folder. This should trigger the sync.

## ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60d362a79e87cb3d01246780/file-1AhXNVOBNt.jpg)

## Using CloudKit Sync

If you are using CloudKit (default), a force sync is not available and not needed, because it automatically syncs as soon as possible.

*Last updated: 2023-09-02*
