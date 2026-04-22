# Plugins

You can install or create JavaScript plugins to add new features to NotePlan. These plugins can add "/commands" to the command bar or open an HTML-based interface in a separate window. They work on both Mac and iOS.

🔎 [Browse, search and install Plugins](https://noteplan.co/plugins)
▶️ [Watch Stacey introducing plugins here.](https://youtu.be/jZYtU1qTE-w)

## **Examples**

****📙 *[Note Helpers](https://noteplan.co/plugins/jgclark.NoteHelpers)** **→ /move note
*Move the currently opened note to another folder without using the mouse for drag & drop

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/60906d648af76a714bfdd9b2/file-de29hk9eGd.gif)

🔢 *[Note Statistics](https://noteplan.co/plugins/np.statistics)* *→ /wc
*Display some details about the currently opened note, such as word, character, and line count:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/608c5894e0324b5fdfd10c06/file-iO2jcIz0qW.png)

## Install Plugins

Install plugins directly in NotePlan's preferences on your Mac or visit our [Plugin Page](https://noteplan.co/plugins). It will download and check for updates in our [official repository](https://github.com/NotePlan) (if you are a developer, you can submit your plugins here).

On iOS, you can open the command bar (tap on the magnifier glass icon bottom right), [then type “/install plugins”](127-install-plugins-on-ios.md).

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/617768fc2b380503dfdfdfe8/file-zIygeUrFoq.png)

Right after downloading you can close the preferences and hit `⌘J`, then type `/` and you should see a list of available commands.

Alternatively, you can type `/` anywhere in the editor to invoke a plugin command. If you want to see all commands in a list along with their plugins, you can use the plugin "Plugin Information & Tester" to generate a listing.

**Where are plugins saved?**

Plugins are saved locally NotePlan's folder and synced between your devices. In the plugin preferences you can open the folder (if you have never installed a plugin, the folder won't be existing yet):

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/608c59038af76a714bfdd0a7/file-xGr6tFaHqb.png)

Each plugin is saved into a separate folder having at least one setup file `plugin.json` and one script file which is defined in the setup file (for example `script.js`):

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/608c59c9e0324b5fdfd10c10/file-447Nj0Acwx.png)

If you are a user and have plugin ideas, [submit them here](https://feedback.noteplan.co/plugins-scripting) or ask in our [Discord community](https://discord.gg/D4268MT) in the `#plugins` channel.

If you are a developer and want to contribute and build your plugins, join other developers on [Discord](https://discord.gg/D4268MT) (`#plugin-dev` channel).

---

**Next up:** [Create Command Bar plugins →](67-create-command-bar-plugins.md)

**Jump to:**

- [Plugins](65-commandbar-plugins.md) *(you're here)*
- [Create Plugins](67-create-command-bar-plugins.md)
- [Javascript Plugin API](70-javascript-plugin-api.md)
- [Plugin Hooks](137-plugin-hooks.md)
- [Releasing a plugin](80-releasing-a-plugin.md)

*Last updated: 2025-01-11*
