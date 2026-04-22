# What information is sent to OpenAI?

If you are using the AI features like transcription or "Prompt AI" using the Command Bar (CMD+J) or through a slash command or CMD+O (on Mac and iPad) some data is sent to OpenAI for processing.

## "Prompt AI"

If you are using "Prompt AI" through the command bar or on iOS using the toolbar "Prompt AI on selection", only the selected text and your prompt are uploaded to OpenAI to generate an answer. If you are transcribing your voice, only the recorded audio file is uploaded for processing.

Nothing else is shared with OpenAI. Your existing notes, or the content of the currently opened note (except the selected text if you are using the AI features), stay private (still synced via Apple's CloudKit / iCloud Drive, if you didn't opt out).

This makes the AI features by default "opt-out". By using them you opt-in sending a very limited amount of information.

### Sending Context

You can send more context with the prompt dialog. If you do this, the content of the selected notes or notes in the selected folders is also sent

## Note Properties AI

At the bottom of the note properties user interface is an AI button. It opens a dialog where you can enter a prompt. Submitting this dialog, the prompt is sent to OpenAI, the content of the currently opened note, and the properties (i.e. Frontmatter text) of all it's "sibling" notes (of all notes in the same folder), but not the content of the notes.

*Last updated: 2025-07-11*
