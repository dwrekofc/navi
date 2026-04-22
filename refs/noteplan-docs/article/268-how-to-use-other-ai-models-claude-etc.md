# How to use other AI Models (Claude, etc.) or Proxy URLs?

*Note: Available from v3.18*

You can change the host url (to use a different AI provider or to use a proxy URL), and AI models used in NotePlan (you will most likely have to change both). This allows you to switch to other models that share the same API format such as Claude Sonnet for example. And it allows you to hardcode which API model should be used (GPT 4.1, o3, etc.).

In most cases you will also need to define a [custom API key.](213-where-do-i-enter-my-openai-key.md)

## Host URL

Open the Command Bar (CMD+J) and type "Set AI Host URL" and a setting should appear, which you can select to open a dialog which allows you to define the host url.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/687141f5be3db56c420583e5/file-0YnWailKAz.png)

## API Model

To define the API model (since say Anthropic / Claude don't use the same model names as OpenAI), you can type into the Command Bar (CMD+J) "Set AI Model", which should show that as an option to select.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6871425f09e05d43a45fb423/file-2ZpAOZQQob.png)

## Voice Transcription Host URL

The voice transcription URL will fallback to OpenAI by default, since other models often don't support something similar to OpenAI's Whisper API structure. But you can set it separately if you are using a proxy URL for instance. Open the Command Bar (CMD+J) and type "Set AI Transcription Host URL".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/687143d3be3db56c420583ed/file-M5riDTwiiS.png)

## Voice Transcription API Key

You can also set an API key specifically for voice transcription, if you have set a different transcription host url. Type "Set Transcription API Key" into the Command Bar (CMD+J).

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/687148b809e05d43a45fb441/file-Dunb6SLbZk.png)

## Voice Transcription API Model

You can also set an API model specifically for voice transcription, if you have set a different transcription host url. Type "Set Transcription Model" into the Command Bar (CMD+J).

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/687148efbe3db56c4205840e/file-mbWE5jEVHg.png)

*Last updated: 2025-07-11*
