# Releasing a Plugin

Users can download plugins in NotePlan's preferences under "Plugins".

When you click on "Plugins", NotePlan will query the [Releases of the Plugin GitHub Repo](https://github.com/NotePlan/plugins/releases) and look for available plugins. All release entries are downloaded, the `plugin.json` files are read and then displayed in a list. More information is displayed when you expand an available plugin, such as the available commands and description.

The `plugin.id` and `plugin.version` are also compared against the locally available plugins (in the `Plugin Folder`). If the same plugin exists locally, an "Update" button will be displayed which will download and overwrite the locally available version.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/617769af2b380503dfdfdfed/file-65VtpbGnyk.png)

## Steps to release or update a plugin

1. You either will need access to the [official plugin repository](https://github.com/NotePlan/plugins) or submit a pull request with the code and a request to release it (so an admin can review and release it for you).
2. To add a new plugin or update an existing plugin, you need to create a new release on the [Releases page of the repo](https://github.com/NotePlan/plugins/releases). ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/609a791845543f49ed69a1ad/file-ByLwZLtqQU.png)
3. Fill out the details for your release and upload the `plugin.json` and `plugin.script` files.

### Release Details

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/609a7a2c45543f49ed69a1b2/file-5P2T8C1SkA.png)

1. Use the `plugin.id` with the version number preferably as a unique tag. Like `eduardme-notehelpers-v1.0`.
2. Name of your plugin. You can also add a version number. This will only appear on the releases page and is not used by NotePlan. NotePlan reads the `plugin.name` from the `plugin.json`.
3. Add a description for the changes. Also, describe here what the plugin does or notices for other developers. This description is not used by NotePlan.
4. **This is the most important part:** Upload your `plugin.json` and the script (`plugin.script`) and dependencies, if any. Don't upload unnecessary files here, besides an optional readme, which could be read if the plugin folder is opened by a user. If you upload no files here, the release will be ignored by NotePlan.
5. Publish your release.

If you want to update an existing plugin, you can create a new release and delete the old version or update the existing version. Just make sure to increment the `plugin.version` inside the new `plugin.json` file. NotePlan uses this field to determine, if an update is available.

You can debug your release using the Plugin Console. First, open the Plugin Console, then click on "Plugins" in the preferences:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/609a7c73bbbb0e786d309fe5/file-cKblWnJy8U.png)

---

**Jump to:**

- [Plugins](65-commandbar-plugins.md)
- [Create Plugins](67-create-command-bar-plugins.md)
- [Javascript Plugin API](70-javascript-plugin-api.md)
- [Plugin Hooks](137-plugin-hooks.md)
- [Releasing a plugin](80-releasing-a-plugin.md)* (you're here)*

*Last updated: 2022-05-30*
