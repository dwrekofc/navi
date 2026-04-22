# Sync with Git

If you can’t use iCloud on your Mac due to security restrictions for example, you can still sync your notes using git.

### Before we start

You need the following accounts and apps:

1. A GitHub (preferred) or BitBucket account.
2. Download and purchase the app “Working Copy” on iOS. You need the premium version so you can push changes to the server.
3. Download a git client on Mac, such as GitHub’s Desktop client (free) or Sourcetree for BitBucket (free). You can also use the terminal, but a client app will make things easier to manage.

### **Setup your repository**

1. On Mac and iOS, run NotePlan at least once, so the notes folder gets created.
2. Set the sync on Mac and iOS inside NotePlan to “No Sync” in the preferences.
3. [Open Finder at NotePlan’s sync folder](31-where-are-my-notes-saved.md) and delete “co.noteplan.NotePlan3”, backing up any files, if you already have some. GitHub can’t create a new repository in an existing folder.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61168f9521ef206e5592b524/file-fbczumfxeZ.png)
4. Open GitHub Desktop or any other git app, create a new repository pointing to the “.../Application Support” directory where we deleted NotePlan’s sync folder and name the folder “co.noteplan.NotePlan3”.
Local Path: `/Users/eduardmetzger/Library/Containers/co.noteplan.NotePlan3/Data/Library/Application Support`
Name: `co.noteplan.NotePlan3`
Make sure to change the user name ("eduardmetzger") in your local path to your Mac's username, if you copy it from above. ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61168ea664a230081ba1f03e/file-9oS9BFvkHr.png)
5. Copy any files back which you have backed up before.
6. Now we need to ignore some folders from syncing, otherwise, they will create conflicts continuously. Ignore the folder “Caches”, “com.microsoft.appcenter”, and files ending with “.log”. You can do this in the GitHub Desktop app under "Repository" > "Repository Settings" in the menubar:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61168fff21ef206e5592b526/file-Upv85WC9SH.png)![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611690366ffe270af2a98ad5/file-F3IJXZrbXM.png)Paste the following in there:
```
Caches*
*.log
com.microsoft.appcenter*
```

7. Make your first push on Mac! First, you need to commit by entering a summary (such as "update") bottom left and clicking on "Commit", then press the push button top right in GitHub Desktop.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611690e0b55c2b04bf6de2fd/file-DMd0LsBHeJ.png)
8. On iOS, close NotePlan and open NotePlan’s folder in the Files app in “On my iPhone/iPad” and delete any files in there, so they don’t cause issues.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61169178b55c2b04bf6de32c/file-BAstGOHPeS.png)
9. Setup Working Copy the same on iOS. Tap on the plus icon at the top left and then on “Setup synced directory” and select NotePlan’s folder in “On my iPad/iPhone”.
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611691d6b37d837a3d0e3991/file-vYfVuGp4K2.png)
10. Add the previous repository we have created on your Mac to this folder and pull changes. First go to your GitHub account in Safari and open the newly created repo, then click on the green "Code" button to copy the "SSH" link:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61169272b37d837a3d0e3995/file-WOKyFbDhOB.png)Then on iOS, in the Working Copy app, tap on the synced NotePlan folder you have added before, now called "Documents", tap at the top on "Repository", then tap on "Add Remote" at the bottom:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611693036ffe270af2a98b02/file-JXU9nK9Thj.png)
Into the URL field, paste or type the SSH URL we have copied before from the GitHub repository website:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6116938021ef206e5592b55b/file-44PoqXQcKm.png)
If you are already authenticated with GitHub, you can tap on "Test". If you are not authenticated, you can tap at the top right on "Save", go back to the first screen in Working Copy and open the preferences by tapping on the preferences icon top left. Inside the preferences, select "SSH Keys":
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/611693f2b55c2b04bf6de334/file-jkTj5K94EM.png)
Tap on the key which should be already available:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6116943721ef206e5592b55e/file-9mVSjuIpA4.png)
Then tap on "Connect with GitHub" and enter your GitHub username, password, etc:
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6116947d64a230081ba1f08a/file-H05140K3di.png)
11. Now you are ready. You can make changes either on Mac and iOS and then you have to manually commit and push the changes. On the other device, you will need to pull changes every time. If you don’t do this, you will run into conflicts, which you can resolve with the git apps easily.

### Your first sync

1. To make your first sync, change a file on your Mac by entering something for example in today's note.
2. Then open GitHub Desktop and you should see some files on the left under "changes".
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61169556b55c2b04bf6de339/file-NH6yF7tBGG.png)As before, enter something into the summary field below and hit "Commit", then "Push origin" at the top right.
3. Now open Working Copy on iOS, select your repository called "Documents" and hit the Working Copy button at the bottom right. From here you can also commit, push and pull. Since we want to pull the changes we did on Mac, we need to select "Pull" and this should download the changes.
4. If you make a change in NotePlan on iOS, you can go back to Working Copy, tap on the floating button bottom right again and this time commit your changes by selecting all files which are listed, hitting "Commit" (type a summary if needed) and then hit "Push".
5. On Mac, you can open GitHub Desktop and pull the changes.

### Resolving Conflicts

You might run into a conflict if you have changed a note on two different devices without pushing/pulling first. In this case Working Copy or GitHub Desktop will notify you of the conflict and you need to resolve it by selecting the version you want to keep or trying to merge it. Then you commit this selection and pull again.

*Last updated: 2021-08-13*
