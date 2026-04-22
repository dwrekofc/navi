# Where is Dropbox Sync?

It is not recommended to use Dropbox sync with NotePlan. The sync implementation is incomplete and we decided not to work on it, because the interface Dropbox provides us for syncing has a very low quota. This means Dropbox servers are throttling the sync speed too quickly to be useful for NotePlan.

If you heavily rely on it and want to use it despite all warnings, you can still unhide the button in the preferences by clicking on following link (on iOS and macOS separately, this is not getting synced across platforms; reopen the preferences after this): [noteplan://x-callback-url/showDropbox](noteplan://x-callback-url/showDropbox.md)

*Last updated: 2023-02-13*
