# Turn off sync conflict detection (red bar)

[Read here](58-turn-off-sync-conflict-detection.md) about how to resolve conflicts and how NotePlan is handling multiple versions of a note.

---

If you experience an unusual amount of sync conflicts using CloudKit, you can turn off NotePlans conflict detection. **However, this is not recommended, as it can lead to data loss because NotePlan will overwrite the note with the most recent version automatically**. Instead, [enable logging and send us the log files](57-how-to-enable-logging.md) first, so we can troubleshoot and fix the problem.

To turn off the conflict detection, open the preferences, click on "Sync" and expand the "Advanced" options of CloudKit. Scroll down and click on the checkbox "Detect Sync Conflicts":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/608828b43149d33a19c70fb7/file-3Qn4jQ3icG.png)

Similar on iOS:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6088291d3149d33a19c70fba/file-Z7D63Kcjw6.png)

*Last updated: 2021-04-27*
