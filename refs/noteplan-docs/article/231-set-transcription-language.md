# Set AI Transcription Language

With version v3.14.2 you can set the transcription language if you run into the issue that the language wasn't properly detected. OpenAI's Whisper sometimes returns the text in Spanish, although you spoke English for example. Sometimes it's enough to mention one word from another language or pronounce it slightly like that.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/670013ba44628317ef904d7a/file-DtHXEeWs3Y.png)

To resolve this issue, OpenAI allows us to set the language. In NotePlan you can set it by opening the Command Bar (CMD+J on macOS and on iOS tap on the magnifier glass icon at the bottom right), then search for "set transcription language", click on it, and type in the language code. For example "en" for English, or "de" for German, "es" for Spanish, and so on. It has to be a valid [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes).

Here are some more details: [https://platform.openai.com/docs/guides/speech-to-text/supported-languages](https://platform.openai.com/docs/guides/speech-to-text/supported-languages)

*Last updated: 2025-07-11*
