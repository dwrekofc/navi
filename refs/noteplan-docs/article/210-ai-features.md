# AI Features / Voice Transcription

Updates:

- [v3.17.2](https://noteplan.co/changelog/v3.17.2): Note Properties AI with custom prompts
- [v3.17.4](https://noteplan.co/changelog/v3.17.4): Updated AI Prompting and voice transcription interface on macOS and contextual AI in general

NotePlan uses OpenAI for transcription and AI responses. Learn in the following videos how you can use the features:

## How to use "Prompt AI" on Mac

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/691e359e18d68872e6a3def0/file-EgO0J4VTSw.gif)

You can use "Prompt AI" to modify existing text or create new text by AI. Use it for example to translate text, expand it, summarize meetings, convert a block of text into bullets, convert it into an email reply, and more. You can also let AI generate text for you, like "Create 10 stoic quotes" and you'll get a list pasted into your note.*
*

**How to use it?**

1. To use this feature, select some text (or set the cursor where you want the new text to appear).
2. Hit CMD+O to open the AI prompt dialog.
3. Type in your prompt, like "Translate into Spanish"
4. Select "Generate".

## How to use transcription on iOS

iOS has the same "Prompt AI" features (here open the command bar by tapping on the magic button, then "prompt on selected text"), but in addition, you can also transcribe text. This lets you add notes, tasks, bullets, etc. to your notes on the go fast and accurately without having to type (and waste time with auto-correct). The voice-to-text transcription has human-level accuracy, so you don't need to worry about filler words like "ehm, eh" and pauses or external noises.

**How to use it?**

1. Set the cursor where you want the transcribed text to appear
2. Tap on the **magic button > "Transcribe Voice"** on the toolbar
3. Talk as short as seconds and as long as minutes about what you want it to add to your note
4. Hit the stop button
5. Select either "Plain Text" if you want it to paste what you said without modifications. If you want AI to process it into a bullet list, task list, or even time blocks, use one of the other options.

**How to plan your day with AI?**

Follow the steps above, then hit "Timeblocks" when you are done talking inside a Daily Note (like today's). To create a complete plan make sure you tell NotePlan what you want to do, when, and for how long. You can go through your complete day. Don't worry about mistakes, if you make one, just tell it in the same session.

The result will be a list of times and tasks which you can see as blocks in the timeline. Open it by tapping on the bottom right calendar icon (inside daily notes).

**How to add custom transcription prompts?**

The list buttons you see at the end of a transcription session can be extended with custom prompts. You just need to write them into any notes in a specific way, here's an example you can copy:

```
#custom-prompt { title: "OKR", icon: "star", color: "#00CEAD", prompt: "I'll tell you my plans and goals and you create a list of OKRs for me."  }
```

Add this into an empty line in any note, then use the transcription feature again and it should appear as an additional button. The structure is quite simple:

Tag: `#custom-prompt`

Followed by a JSON:

`{ title: "<write your title here>", icon: "<name for the SF Symbol icon>", color: "<color as hexadecimal>", prompt: "<custom prompt>" }`

The title and the prompt are the most important elements. The rest is optional. You can use the [SF Symbols](https://developer.apple.com/sf-symbols/) app from Apple to look up icons.

Example:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/64b54e50a9d61472afe08da6/file-j94bCJXUFC.png)

## AI in Templates and Plugins

The plugin API has an AI command:

```
await NotePlan.ai(prompt, [list, of, notes])
await NotePlan.ai("Return a bullet. Not just the first one, make it random", ["this year"])
```

This is also available in [templates](233-ai-prompts-in-templates.md).

*Last updated: 2025-11-19*
