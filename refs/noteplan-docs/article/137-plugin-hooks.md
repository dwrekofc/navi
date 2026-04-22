# Plugin Hooks

NotePlan calls certain functions in your plugin when the user performs some action so you can run some preparations before your plugin is used (like migrating data). For example, when the plugin gets installed or updated or when the user saves the settings for this plugin.

## init

This function is called before the user executes any command of a specific plugin.

```
function init() {
  console.log("Initialize the plugin before the command is executed")
}
```

##

## onUpdateOrInstall

This hook is called after the user has either updated or installed the plugin.

```
function onUpdateOrInstall() {
  console.log("Plugin updated or installed, perform migration or other preparations now")
}
```

## onSettingsUpdated

This hook is called when the user has opened the configuration of the plugin and clicked on "save & close" and when the setter of `DataStore.settings` was called or `DataStore.saveJSON(object)` was called to save settings from within the plugin.

```
function onSettingsUpdated() {
  console.log("Settings updated of this plugin, run cleanup, or other preparations")
}
```

---

**Next up:** [Releasing a plugin →](80-releasing-a-plugin.md)

**Jump to:**

- [Plugins](65-commandbar-plugins.md)
- [Create Plugins](67-create-command-bar-plugins.md)
- [Javascript Plugin API](70-javascript-plugin-api.md)
- [Plugin Hooks](137-plugin-hooks.md) *(you're here)*
- [Releasing a plugin](80-releasing-a-plugin.md)

*Last updated: 2022-05-10*
