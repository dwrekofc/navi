# Use NotePlan with Obsidian

NotePlan and Obsidian are both saving your notes as plain text markdown files. So you can use them in combination by opening NotePlan's folder as a vault in Obsidian.

Below the are basic steps to make both apps work together. If you want to go deeper, [read this very detailed article on this topic by our friend Ryan.](https://fulcra.design/Posts/Working-with-Obsidian-and-NotePlan/) Additionally, learn here how to [make daily notes and NotePlan’s trash folder work better together with Obsidian](https://www.reddit.com/r/noteplanapp/comments/p5perm/wanted_to_share_better_bidirectional_setup_w/). And in [this article](https://docjulien.medium.com/the-powerful-combo-of-obsidian-and-noteplan-3-4e03b84ee5d9) learn how to combine Obsidian and NotePlan in order to use data views.

### 1. Change the default file extension of your notes to ".md"

NotePlan saves your notes as ".txt" files by default. You first need to change the default file extension. You can do this in NotePlan's preferences under "Files". Type in "md" and click on Apply.

Change it both, on iOS and Mac. Internally, NotePlan renames all your notes and reuploads them. You need to do this on every device once:

### ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6089b0573149d33a19c71a69/file-SUZ637DDZS.png)2. Point Obsidian to NotePlan's folder

In Obsidian open NotePlan's folder as a vault (navigate to it or drag the folder into the picker). [Read here](31-where-are-my-notes-saved.md) how you can find out where NotePlan saves your notes.

From [Ryan's article](https://axle.design/why-and-how-to-use-obsidian-and-noteplan-together):

````````

> In NotePlan on your Mac, open your sync Advanced options. Select “Open Local Database Folder.” This will open the location of your NotePlan notes in the Finder.
> In Obsidian, open the Vault Picker. This happens by default when you first run the app, but if you already have a vault, you can open it by clicking on the little vault icon (on the bottom-left menu bar in the default theme) or by going into the command palette (cmd+p) and searching for “Open another Vault”. Then, choose “Open Folder as Vault”. In the Finder window that shows up, navigate to the NotePlan Local Database Folder we found above. (The easiest way to do this may simply be to drag a folder from the Finder window we opened in NotePlan a moment ago into the Choose Folder pop-up opened by Obsidian.)
> This will create a vault in Obsidian using the same folders NotePlan uses. Two of these folders are meaningless in Obsidian: one called
> com.microsoft.appcenter
> and another called
> Filters
> . Ignore them. The other two folders are intuitive. The
> Calendar
> folder is where your Daily Notes will go. The
> Notes
> folder is for everything else.

First open NotePlan's folder (in this example we assume you use CloudKit sync, otherwise you need to open the iCloud Drive folder of NotePlan):

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60b20a929c887a0dfc554b14/file-jl5IsxItim.gif)

Then open a NotePlan's folder as vault in Obsidian:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60b20a594dda6972e09308bd/file-USzZ1MLC2L.gif)

### 3. Keep filenames and titles in sync

NotePlan doesn't (yet) update your filenames if you change the note title. That's because NotePlan uses the note titles (first line in the note) as the references, whereas Obsidian uses the filename. So if you want to use linking and the graph in Obsidian to view your NotePlan files, you need to keep the note title and the filename the same as of now. This might change in the future, though. [Follow this feature request to stay up to date](https://noteplan.canny.io/general-feature-request/p/choose-note-reference-filename-or-first-line)

### 4. Daily Notes

NotePlans primary way to schedule tasks is `>YYYY-MM-DD`, but `[[YYYY-MM-DD]]` is also supported and nothing needs to be changed in this case. But your notes will only be detected if they follow the filename format `YYYYMMDD.md`. You will need to rename your daily notes first. The easiest way is to use the "rename" function of Finder. Select all notes, right-click on the selection and select "rename":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6310e531037bc877147b5485/file-heGNUbxBgP.png)

Choose: "Replace Text" in the drop-down top left, insert "-" into "Find:" and leave "Replace with:" empty.

*Last updated: 2023-04-28*
