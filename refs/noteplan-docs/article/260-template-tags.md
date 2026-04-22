# Template Tags

Templates use a tagging system to indicate where dynamic content will be generated within template notes.

## Tags

- `<%` Script tag to be used when executing any JavaScript command without producing any output
- `<%-` Outputs the value into the template (most used tag)
- `<%_` 'Whitespace Slurping' Template tag, strips all whitespace before it
- `<%#` Comment tag, no execution, no output
- `<%%` Outputs a literal `<%`
- `-%>` 'Whitespace Slurping' Template tag, strips all whitespace after it
- `%>` Plain ending tag

## Output Tags

When you wish to output anything to the rendered template, you use the output `<%-`

## Examples

The following are various examples of `Templating` tags in action

**JavaScript / Flow Control Tag**

The basic flow control tag `<%` is used when you want to perform a standard JavaScript action such as setting a variable, doing a calculation, looping or conditionals. In this example, a `getData` method would be called, but the actual output would be displayed in another section of template. If you have many lines of JavaScript, you may want to consider the [templatejs code block](254-embedded-template-code-templatejs.md).

```
// the variable is set in this tag
<% const data = getData() %> 
... 
// the output is output later in this tag
<%- data %>
```

Note that flow control tags, like all other tags are on a line by themselves. This means there is a return at the end of the line. When the template is rendered, the tag will be processed and deleted, but the return at the end of the line will show up in your rendered template as a blank line. If you don't want the returns, use the minus (slurping tag) at the end of each flow control line, e.g.
```
// there will be no blank line when this is processed because of the dash at the end.
<% const myVar = note.title -%>
```

### Standard output tag (variable)

Display `first` name variable contained in `Templating` Plugin Settings.

```
<%- user.first %>
```

### Standard output tag (module method)

Displays current date from Date Module.

```
<%- date.now() %>
```

### Unescaped Output Tag

Displays result from `templates.services.developerQuote` defined in `Templating` Plugin Settings

[](https://noteplan.co/templates/docs/docs/settings)`Templating` doesn't escape characters by default. When doing web requests, it may be useful to escape dangerous characters. You can escape a command's response characters using the `<%-` tag.

```
<%- web.service('developerQuote') %>
```

### Strip Whitespace

When you have have process tags (this which do no produce output), it is recommended that you use the `Whitespace Slurping` tags.

```
<%_ const testName = 'Mike' -%> name: <%- testName %>
```

will produce the following output

```
name: Mike
```

###

## [      ](https://noteplan.co/templates/docs/templating-tags#tags)
[](https://noteplan.co/templates/docs/templating-tags#tags)

[](https://noteplan.co/templates/docs/templating-tags#tags)

[](https://noteplan.co/templates/docs/templating-tags#tags)

[](https://noteplan.co/templates/docs/templating-tags#tags)

[](https://noteplan.co/templates/docs/templating-tags#tags)

[](https://noteplan.co/templates/docs/templating-tags#tags)

[](https://noteplan.co/templates/docs/templating-tags#tags)

[](https://noteplan.co/templates/docs/templating-tags#tags)

*Last updated: 2025-05-18*
