# Is my data secure (encrypted)?

For syncing purposes, your data is stored on CloudKit (iCloud) and locally on your device (so you have offline access). Apple's iCloud servers use [encryption](https://developer.apple.com/documentation/cloudkit/encrypting_user_data) to ensure that only authorized users can access data. We are not using an additional layer of encryption. The data is encrypted in transit by TLS (HTTPS).

**Note:** Your attachments, like images and PDFs, etc. are stored as CloudKit "Assets", which Apple documents are encrypted by default.

As the developer, we have no access to your data. Your data can be accessed only with your private Apple ID login.

- Read about [CloudKit Security](https://support.apple.com/guide/security/cloudkit-security-sec3d52c0374/1/web/1).
- Read about [CloudKit end-to-end encryption](https://support.apple.com/guide/security/cloudkit-end-to-end-encryption-sec3cac31735/1/web/1).
- Read about [iCloud Drive security](https://support.apple.com/guide/security/icloud-drive-security-sec0bc781c0d/1/web/1).
- Read about [iCloud security overview](https://support.apple.com/en-us/HT202303).

Additionally, to secure the locally saved data, you can enable "FileVault" in your system preferences:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/609a8cb1a6f4b5439f4af64b/file-vJe7yIYqFv.png)

You can also enable an extra layer of encryption for your notes by turning on "Save Note Content as Encrypted Asset". To find this option:

1. Open Settings.
2. Click on **Sync**.
3. Below "Use CloudKit", click the **Advanced** button to reveal more options.
4. At the top you will find the **"Save Note Content as Encrypted Asset"** option.

**Important:** Enabling encryption can slow down sync, which might be noticeable if all notes need to be downloaded, as each note needs to be encrypted and decrypted individually.

This will store the contents of your notes as an "asset", which is by default encrypted by Apple on their servers. This includes the title of the note, but not the filename. Read more about this encryption method [here](https://developer.apple.com/documentation/cloudkit/encrypting_user_data#3813930).

*Last updated: 2026-03-23*
