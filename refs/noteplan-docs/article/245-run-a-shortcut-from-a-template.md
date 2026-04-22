# Run a Shortcut from a Template

Shortcuts can be opened through an URL: "shortcuts://run-shortcut?name="

And templates allow you to open URLs through the API:

```
<% NotePlan.openURL("shortcuts://run-shortcut?name=Hello%20World") -%><br>
```

You can copy this line into your template text and modify the value behind name= to open your specific Shortcut. Note: You need to convert spaces to %20 and other special character, you can use tools like [this](https://www.urlencoder.org/) one.

*Last updated: 2025-05-18*
