# Plugin Configuration

You can add settings to your plugin by extending the `plugin.json` file with an array `plugin.settings`. Each item in this array represents one setting in the configuration UI with title, input, and description (note: available since v3.3.2).

## Settings Schema

Below is a list of all supported settings keys. Note, they are case-sensitive.

*(* fields are required)*

- **key**: * unique key the value will be assigned to
- **type**: * data type which will determine which control is being displayed
- `string`, displays a text box for strings
- `[string]`, displays a text box expecting comma-separated strings which will be turned into an array
- `number`, displays a text box for numbers
- `date`, displays a date picker
- `bool`, displays a check box
- `json`, displays a text box for strings which will be converted to a JavaScript object (note: use a string for the default value)
- `separator`, displays a horizontal line (visual element only)
- `heading`, displays a heading, so you can segment your settings page (visual only),
- `hidden`, displays nothing, it's a hidden string setting
- **default**: the default text prefilled in the input box and used as initial settings value (it has to be a string value for all keys at the moment)
- **required**: true will not allow the user to save the config without value
- **choices**: array of options which will be displayed in a dropdown
- **title**: short title of the setting (displayed above)
- **description**: lengthy description shown below the setting
- **boxHeight**: applies to string, number, and JSON and allows you to set the height of the input box

## Accessing Settings

Calling `DataStore.settings` in the plugin code will return a JavaScript object with the key/value pairs from the settings array. In case the user has ever saved any settings, the values will be available here. If the settings have never been saved before, the default values from the settings array will be populated. This is also a setter, so you can assign a JavaScript object which will be saved into the settings file.

## Migrating Settings

After the user has updated or downloaded the plugin, NotePlan attempts to call a function `onUpdateOrInstall()` in your script.js code. If it doesn't exist, it will silently fail with an error message in the Plugin Console window. Use the `onUpdateOrInstall()` function to create a settings file from existing user settings if any.

Here's a simple example of the function (use the `async` keyword if you are going to use `await`):

```
async function onUpdateOrInstall() {
	// TODO: Load existing settings into an object
	// TODO: Write settings to DataStore.settings or a custom json file with DataStore.saveJSON(object, filename?)
	// TODO: Delete old settings
}
```

## Example

The following schema covers all available types. You can use it as a reference and starting point.

```
"plugin.settings": [
	{
	     "type": "hidden",
	     "key": "hiddenKey",
	     "default": "hello world"
	},
        {
            "type": "string",
            "key": "someString",
            "title": "String Setting",
            "description": "An example of a string setting.",
            "default": "What a value",
            "required": true
        },
        {
            "type": "number",
            "key": "someNumber",
            "title": "Number Setting",
            "description": "An example of a number setting.",
        },
        {
            "type": "separator"
        },
        {
            "type": "heading",
            "title": "Heading"
        },
        {
            "type": "bool",
            "key": "someBool",
            "title": "Boolean Setting",
            "description": "An example of a boolean setting."
        },

        {
            "type": "date",
            "key": "someDate",
            "title": "Date Setting",
            "description": "An example of a date setting.",
            "format": "YYYY-MM-dd"
        },
        {
            "type": "separator"
        },
        {
            "type": "heading",
            "title": "Heading"
        },
        {
            "type": "[string]",
            "key": "someStringArray",
            "title": "String Array Setting",
            "description": "An example of a string array setting.",
            "default": [
                "one",
                "two",
                "three"
            ]
        },
        {
            "type": "separator"
        },
        {
            "type": "string",
            "key": "someStringEnum",
            "title": "String Setting With Choices",
            "description": "An example of a string setting with choices.",
            "choices": [
                "one",
                "two",
                "three"
            ]
        },
        {
            "type": "json",
            "key": "shortcutExpenses",
            "title": "Shortcut Expenses",
            "description": "Just some example shortcut expenses - please adapt to your needs. You can also add an amount, then you can insert the shortcut without any question.",
            "default": "[\n\t{\n\thello: \"world\"\n\t}\n]",
            "required": true,
            "boxHeight": 300
        },
]
```

This will create the following configuration UI when the user clicks on the gear icon in the plugin overview in the preferences:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/61daef5ba0e8d7327cfaa2d5/file-V8o46CxzyB.png)

*Last updated: 2022-05-30*
