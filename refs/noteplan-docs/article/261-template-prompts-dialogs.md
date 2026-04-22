# Template Prompts (Dialogs)

Templating Prompts Documentation
`Templating` provides the ability to ask the user questions through prompts when rendering templates.

- [Example 1: Simple text input Prompt](#example-1)
- [Example 2: Prompt with list of choices](#example-2)
- [Example: Define early; use later](#example-define-early)
- [Asking for dates or date intervals](#asking-dates)
- [Working with Frontmatter Keys and Values](#working-frontmatter)
- [Working with Tags and Mentions](#working-tags)
- [Usage Tips](#usage-tips)

### Single Quotes

Use single quotes inside the prompt command, like `prompt('question')`.

## Example 1: Simple text input Prompt

For example, if you have a display tag `<%@` in your template which is not in your template data, a prompt will be displayed

```
<%- prompt('What is your first name?') %>
```

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67f13c22cfee38650144efc1/file-uoizuZKb0s.png)

## Example 2: Prompt with list of choices

Alternatively, the **`prompt` command** can accept optional prompt message and well as choices (for use with choice list prompt)

### PROMPT PLACEHOLDER

When using `prompt` command, you must supply a valid placeholder name (e.g. `name`) and the variable must contain valid characters:

- must start with an alpha character (a..z, A..Z)
- may only contain alphanumeric characters (a..z, A..Z, 0..9)
- may **not** contain spaces

Using the following template

```
Task Priority: <%- prompt('priority','What is task priority?',['high','medium','low']) %>
```

You can then use the same variable anywhere else in template `<%- priority %>`. When the template is rendered, it will display a choice list prompt

## ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67f13c4b8cf71f547ecbd977/file-qUg3h7LABN.png)

## Example: Define early; use later

The following example demonstrates how you can place prompts at the top of templates, and then use somewhere else in the template

```
<% prompt('lastName','What is your last name?') -%>

The rest of this could be your template code
And then finally use the `lastName` variable
<%- lastName %>
```

The template would render as follows, with the `lastName` value result from prompt on first line (assuming entered `lastName` Erickson)

```
The rest of this could be your template code
And then finally use the `lastName` variable
Erickson
```

## Asking for dates or date intervals

There are two further commands available:

- `promptDate('question','message')`, which accepts dates of form `YYYY-MM-DD`
- `promptDateInterval('question','message')`, which accepts date intervals of form `nnn[bdwmqy]`, as used and documented further in the [Repeat Extensions](jgclark.RepeatExtensions) plugin.

Both require the first parameter to be 'question', but accept an optional prompt message. They must be placed where the text is to be used. For example:

```
Project start date: <%- promptDate('question','Enter start date:') %>
Review frequency: <%- promptDateInterval('question','Enter review interval:') %>
```

## Working with Frontmatter Keys and Values

### promptKey

`promptKey` allows you to prompt the user to select a value from existing frontmatter keys in your notes.

#### Syntax

```
<%- promptKey('key', 'message', 'noteType', caseSensitive, 'folder', fullPathMatch, ['options']) %>
```

#### Parameters

- **key** (string): The frontmatter key to search for values (required)
- **message** (string): Custom prompt message to display to the user (optional)
- **noteType** (string): Type of notes to search - 'Notes', 'Calendar', or 'All' (default: 'All')
- **caseSensitive** (boolean): Whether to perform case-sensitive search (default: false)
- **folder** (string): Folder to limit search to (optional)
- **fullPathMatch** (boolean): Whether to match the full path (default: false)
- **options** (array): Array of predefined options to show instead of extracting from frontmatter (optional)

#### Examples

Basic usage:

```
Project status: <%- promptKey('projectStatus', 'Select project status:') %>
```

With folder restriction and case sensitivity:

```
Tag: <%- promptKey('tags', 'Select a tag:', 'Notes', true, 'Projects') %>
Tag: <%- promptKey('tags', 'Select a tag:', 'Notes', true, 'Work/Projects/Clients', true) %>
```

With predefined options:

```
Priority: <%- promptKey('priority', 'Set priority:', 'All', false, '', false, ['High', 'Medium', 'Low']) %>
```

## Working with Tags and Mentions

### promptTag

`promptTag` allows you to prompt the user to select from existing hashtags in your notes or create a new one.

#### Syntax

```
<%- promptTag('Select a hashtag:', 'includePattern', 'excludePattern', allowCreate) %>
```

#### Parameters

- **promptMessage** (string): The message to display in the prompt (required)
- **includePattern** (string): Regex pattern to include only matching hashtags (optional)
- **excludePattern** (string): Regex pattern to exclude matching hashtags (optional)
- **allowCreate** (boolean): Whether to allow creating a new hashtag if not found (default: true)

#### Examples

Basic usage:

```
<%- promptTag('Select a hashtag:') %>
```

Filter tags to include only those containing "project":

```
<%- promptTag('Select a project tag:', 'project') %>
```

Filter to include "priority" tags and exclude "low" tags:

```
<%- promptTag('Select priority:', 'priority', 'low') %>
```

Don't allow creating new tags:

```
<%- promptTag('Select from existing tags only:', '', '', false) %>
```

### promptMention

`promptMention` allows you to prompt the user to select from existing @ mentions in your notes or create a new one.

#### Syntax

```
<%- promptMention('Select a mention:', 'includePattern', 'excludePattern', allowCreate) %>
```

#### Parameters

- **promptMessage** (string): The message to display in the prompt (required)
- **includePattern** (string): Regex pattern to include only matching mentions (optional)
- **excludePattern** (string): Regex pattern to exclude matching mentions (optional)
- **allowCreate** (boolean): Whether to allow creating a new mention if not found (default: true)

#### Examples

Basic usage:

```
<%- promptMention('Select a person:') %>
```

Filter mentions to include only those containing "team":

```
<%- promptMention('Select a team member:', 'team') %>
```

Filter to include "client" mentions and exclude "former" clients:

```
<%- promptMention('Select client:', 'client', 'former') %>
```

Don't allow creating new mentions:

```
<%- promptMention('Select from existing mentions only:', '', '', false) %>
```

## Usage Tips

- When using `includePattern` and `excludePattern`, these are converted to regular expressions, so you can use regex syntax for more advanced filtering.
- The `allowCreate` parameter is particularly useful when you want to limit selections to existing values only.

*Last updated: 2025-05-18*
