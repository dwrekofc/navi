# Create Plugins

Jump ahead to:

- [Plugin Setup](#pluginsetup)
- [plugin.json](#pluginjson)
- [plugin.script ("script.js")](#pluginscript)
- [Installing Your Plugin](#install)
- [More Complex / Efficient Plugin Development](#advanced)
- [Logging](#logging)
- [Interfaces (Global Objects)](#interfaces)
- [Javascript Promises](#promises)
- [Fetch](#fetch)
- [Syncing Plugins](#syncing-plugins)

---

You can write custom Javascript plugins and share them with the community by submitting a pull request to [our official GitHub repo](https://github.com/NotePlan/plugins). Find example of other plugins in the same repo. This article walks you through how to start creating a plugin.

## [Plugin Setup](#pluginsetup)

A plugin is made up of at least two files: `plugin.json` and `script.js` (the name is defined in the plugin.json).

The `plugin.json` file contains the meta-information about the plugin like id, author, name, etc. and it defines which commands will be available in NotePlan's Command Bar.

The `script.js` contains the actual code. NotePlan will call specific functions in this call, depending on which command was executed.

All plugin files have to be inside NotePlan's database folder. Here is an example of the folder structure:

- `/root`
- Plugins
- Plugin-name // one folder per plugin, the name is expected to the `plugin.id` below
- `plugin.json` // metadata
- `script.js` // the script file containing the code)
- other supporting files, like other Javascript dependencies.

The easiest way to find the folder is to open it from the plugin preferences by clicking on "Open Plugin Folder".

## [plugin.json](#pluginjson)

Here is an example of a minimal `plugin.json` file:

```
{
  "noteplan.minAppVersion": "3.0.21", 
  "macOS.minVersion": "10.15.7",
  "iOS.minVersion": "14",
  "plugin.id": "eduardme.move", 
  "plugin.name": "Move selected note to another folder", 
  "plugin.description": "Move a note from the current folder into another.", 
  "plugin.author": "Eduard Metzger",
  "plugin.url": "https://&hellip;",
  "plugin.version": "0.0.2", 
  "plugin.dependencies": [ ], 
  "plugin.script": "script.js",
  "plugin.lastUpdateInfo": "",
  "plugin.requiredFiles": "",
  "plugin.commands": [ 
    {
      "name": "move", 
      "alias": ["m", "mv"],
      "description": "Moves note to another folder", 
      "jsFunction": "moveNote",
      "arguments": ["note title", "heading"],
      "sidebarView": {
        "windowID": "my-custom-id",
        "title": "My Plugin",
        "icon": "calendar",
        "iconColor": "blue-500"
      }
    },
  ],
  "plugin.settings": []
}
```

- `noteplan.minAppVersion` Set the minimum NotePlan version number this plugin works on. The plugin API is constantly extended, so older versions might not have the API features you are using. Version numbers will be documented in the API, when a change is made.
- `macOS.minVersion` This defaults to macOS 10.15.7 (latest Catalina version) if you don't define it. Older macOS versions have outdated Javascript engines so that new JS syntax is not supported. The minimum version supported by NotePlan is 10.12 (Sierra), but this doesn't support async/await. Use a transpiler to support older JS versions and set the macOS.minVersion to "10.13" if you are using async/await, otherwise use "10.12" as the minimum version. Set nothing and "10.15.7" is assumed.
- Note: In NotePlan 3.0.23 it still defaults to macOS 11 (Big Sur).
- `iOS.minVersion` Similar to the macOS minVersion, but for iOS. Version 14 supports all plugins, lower versions might not anymore. But this is less critical as most phones are updated quickly, while relatively many Macs are not updated due to old hardware.
- `plugin.id` should be unique and identical to the folder which is containing the plugin files. If you plan to submit your plugin (please do!), we need to keep the IDs unique across the repository, so that NotePlan can load them properly. If the plugin is downloaded through NotePlan's preferences, it will use the `plugin.id` to create and name the folder automatically.
- `plugin.name`, `plugin.author`, `plugin.url` (website of your plugin), and `plugin.description` will be used in the plugin preferences in NotePlan to describe the released versions.
- `plugin.version` is important when the plugin is available as release on the repo. NotePlan will read this field and compare it to the local version. Use semantic versioning and avoid using anything other than numbers and periods, like `"1.4.1"`.
- In `plugin.dependencies` you can add the file names of third-party Javascript libraries. They have to be in the same folder as the plugin.json (which means you need to download and add them to the folder). NotePlan will load them at runtime.
- `plugin.script` defines the name of the Javascript file that NotePlan will load to execute the commands.
- `plugin.lastUpdateInfo` (Optional) This text field should describe what major changes are present in the latest update of this plugin. A user who is using this plugin will see this message when the plugin auto-updates (when the user runs a plugin command).
- `plugin.requiredFiles` (Optional) List of files so that you can have extra files copied to the plugin folder when your plugin is packaged (e.g. for use in an HTML window opened by the plugin). Any files to be copied should be in your: `<plugin-root-folder>/requiredFiles` folder. A filename listed in the `plugin.requiredFiles` array will be copied from the `requiredFiles` folder into your plugin root. NOTE: subdirectories are not legal inside the `requiredFiles` or in the plugin folder.
- `plugin.commands` is an array containing the list of available `/commands`. NotePlan will load this list and make it available in the Command Bar as auto-complete options.
- Each command refers to a function in the `plugin.script` file. NotePlan will call this function when the user executes the `/command`. You can have multiple commands per plugin. Ideally, the commands are related.
- `"name"` = Name of the "/command" (don't include the "/" here) which will be available to the user.
- `"alias"` = (Note: available from v.3.0.26, backward compatible) Optional string array of alternative keywords when searching for the plugin. The user can search for the name "move", in this example or "mv" to find the command.
- `"description"` = Short description of what the command does which will be displayed in the Command Bar's list of commands which appears when the user starts to type the "/name" into the Command Bar.
- `"jsFunction"` = The exact function name NotePlan should call which is available in the file "plugin.script" file.
- `"hidden"` = You can optionally hide commands from the UI and the command bar, but still call them from within other plugins.
- `"arguments"` = Optional list of descriptions of each argument the JavaScript function behind this command supports. You can get the list of arguments using `DataStore.listPlugins()` for example and show them to the user when you call this command from another plugin.
- `"sidebarView"` = Optional (v3.20.1) configuration for HTML view plugins that use `HTMLView.showInMainWindow()`. When configured, users can pin this command to the sidebar for quick access. The view will automatically load when the user clicks the pinned item in the sidebar.
- `"windowID"` = Optional custom window ID. Must match the `id` option passed to `showInMainWindow()` in your JavaScript code. If omitted, an ID is auto-generated based on the plugin ID and title.
- `"title"` = Optional display title for the sidebar entry. Falls back to `plugin.name` if not provided.
- `"icon"` = Optional Font Awesome icon name (e.g., `"calendar"` or `"fa-calendar"`). Falls back to `plugin.icon` if not provided.
- `"iconColor"` = Optional icon color as a Tailwind color name (e.g., `"blue-500"`) or hex string (e.g., `"#3b82f6"`). Falls back to `plugin.iconColor` if not provided.
`plugin.settings` is an array with settings that will be used by NotePlan to build a configuration UI, so the user can configure the plugin. [See more details here](123-plugin-configuration.md) (available since v3.3.2).

## [plugin.script ("script.js")](#pluginscript)

This is the Javascript file containing the actual code NotePlan will run. It consists of at least one or more functions which will be called by NotePlan after the user has executed the referenced `/command`. Here is an example file:

```
function moveNote() {
    console.log("Hello World")
}
```

The function name `moveNote` has to be defined in the `plugin.json` as one of the available commands.

## [Installing Your Plugin ](#install)

Once you've written your basic plugin, you need to put it somewhere that NotePlan can find it. Go to NotePlan > Preferences > Plugins and click the button "Open Plugins Folder":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62d1b326c74a080359c8a3b8/file-30kEEEQ4Bs.png)

Create a new folder in that directory (you can name it whatever you want), and drop the files you created (script.js and plugin.json) into that folder. The next time you open the Command Bar and type "/", your new plugin will be loaded and you should see and be able to select your new command (whatever you put for "plugin.command" in the plugin.json).

### [More Complex / Efficient Plugin Development](#advanced)

The instructions above work fine for a basic plugin, but there is a lot of tooling in the [GitHub repository](https://github.com/NotePlan/plugins) that will make you more efficient in creating more complex plugins. After you've acquainted yourself with the basics of plugin development on this page, we highly recommend following the instructions on [GitHub](https://github.com/NotePlan/plugins).

## [Logging and Debugging](#logging)

For simple debugging purposes, you can use `console.log(string)` inside your JavaScript code. Before running the command, open the Plugin Console under "Help" > "Plugin Console" in the menubar:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/608c81cc3149d33a19c72a09/file-kqtXrLHcJV.png)

## [Interfaces (Global Objects)](#interfaces)

Inside the `plugin.script` file, NotePlan provides various global objects as an interface to your notes, the editor, and to NotePlan in general. With these interfaces, you can read data (like the content of a note), prompt user input, and change the data. For example `Editor.content` returns you the current text in the editor and lets you set the text if you call `Editor.content = "text"`.

You can find examples of how to use the interfaces in the [API documentation](70-javascript-plugin-api.md). In the next article, we will list and explain the various global objects and how to use them.

## [Javascript Promises](#promises)

[Promises](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) are supported and you need to use them in order to wait for user input for example. The preferred syntax is to use `await` before the Promise function. Note: Functions that use Promises with await have to be prefixed with `async`. Here is an example:

```
async function selectFolder() {
    let folder = await CommandBar.showOptions(DataStore.folders, "Select")
    // Use folder.value or folder.index [...]
}
```

## [ Fetch ](#fetch)

Use fetch to get data from the internet, like weather information, or connect to the API of another app to sync data between NotePlan and the app. Fetch supports the following options parameters: "method" (like "GET"), "headers", and "body" (which takes a string as value).

Note: Fetch only works with websites that have an SSL certificate installed. This means "https://" must work.

Here are examples:

```
async function weatherFetch() {
    let response = await fetch("https://api.weatherapi.com/v1/current.json?key=f4c442d67eef4574b99184324211404&q=Berlin&aqi=no")
    console.log(response)
}

async function fetchWithParameters() {
  const response = await fetch('https://api.ninox.com/v1/teams', 
  { method: 'GET',
    headers: {
      'Authorization': 'Bearer cfda1581-1a36-11ec-b44d-f9d8217e4195', 
      'Content-Type': 'application/json'
    }
  })
  console.log(response)
}
```

## [Syncing Plugins](#syncing-plugins)

Avoid adding a node_modules folder or any other large folder with many files to the plugins directory. Syncing will probably max out. Additionally, NotePlan v3.3 will exclude node_modules folders from syncing it with CloudKit.

---

**Next up:** [Javascript Plugin API →](70-javascript-plugin-api.md)

**Jump to:**

- [Plugins](65-commandbar-plugins.md)
- [Create Plugins](67-create-command-bar-plugins.md) *(you're here)*
- [Javascript Plugin API](70-javascript-plugin-api.md)
- [Plugin Hooks](137-plugin-hooks.md)
- [Releasing a plugin](80-releasing-a-plugin.md)

*Last updated: 2026-01-15*
