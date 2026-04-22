# Add web services to your templates (such as the bitcoin quote)

Using templates you can pull information from the web directly into your notes. Some web services are already supported such as the weather, affirmations, and quotes.

Here is how you can use them in your [templates](136-templates.md):

```
---
title: Daily Note w/ Web Services
type: empty-note
---
Weather: <%- web.weather() %>

Affirmation:
> "<%- web.services('affirmation','affirmation') %>"
Advice:
> "<%- web.advice() %>"
Quote:
> "<%- web.quote() %>"
```

However, you can also extend the list of supported web services and add information such as stock or bitcoin prices in your daily notes:

1. Find a web service that returns the information in plain text or as a JSON object (note: this website must have an SSL certificate installed, i.e. a working "https" version). For example [the current bitcoin prices from coindesk](https://api.coindesk.com/v1/bpi/currentprice.json).
2. Open NotePlan's Preferences > Plugins and download the "Templating" plugin if you haven't already.
3. Open the settings of the "Templating" plugin and scroll down to the web services.
4. Add your web service.
5. Use your web service in your templates.

### 1. Find a web service

You can google for API access to the information you want to fetch from the web. Some websites require you to sign up and get an API key. Others return the information without any roadblocks, such as [https://api.coindesk.com/v1/bpi/currentprice.json](https://api.coindesk.com/v1/bpi/currentprice.json) (paste this into your browser to test it) returns:

```
{"time":{"updated":"Jun 1, 2022 16:01:00 UTC","updatedISO":"2022-06-01T16:01:00+00:00","updateduk":"Jun 1, 2022 at 17:01 BST"},"disclaimer":"This data was produced from the CoinDesk Bitcoin Price Index (USD). Non-USD currency data converted using hourly conversion rate from openexchangerates.org","chartName":"Bitcoin","bpi":{"USD":{"code":"USD","symbol":"$","rate":"30,710.6775","description":"United States Dollar","rate_float":30710.6775},"GBP":{"code":"GBP","symbol":"&pound;","rate":"24,407.1574","description":"British Pound Sterling","rate_float":24407.1574},"EUR":{"code":"EUR","symbol":"&euro;","rate":"28,663.2274","description":"Euro","rate_float":28663.2274}}}
```

This looks better if you "prettify" or format it using some [website](https://jsonformatter.curiousconcept.com):

```
{
   "time":{
      "updated":"Jun 1, 2022 16:01:00 UTC",
      "updatedISO":"2022-06-01T16:01:00+00:00",
      "updateduk":"Jun 1, 2022 at 17:01 BST"
   },
   "disclaimer":"This data was produced from the CoinDesk Bitcoin Price Index (USD). Non-USD currency data converted using hourly conversion rate from openexchangerates.org",
   "chartName":"Bitcoin",
   "bpi":{
      "USD":{
         "code":"USD",
         "symbol":"$",
         "rate":"30,710.6775",
         "description":"United States Dollar",
         "rate_float":30710.6775
      },
      "GBP":{
         "code":"GBP",
         "symbol":"&pound;",
         "rate":"24,407.1574",
         "description":"British Pound Sterling",
         "rate_float":24407.1574
      },
      "EUR":{
         "code":"EUR",
         "symbol":"&euro;",
         "rate":"28,663.2274",
         "description":"Euro",
         "rate_float":28663.2274
      }
   }
}
```

The interesting information is nested under `"bpi.USD.rate"`.

### 2. Download the "Templating" plugin

Open NotePlan's Preferences > Plugins and download or update the "Templating" plugin if you haven't already.

### ![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62979285e1d2cf0eac00cf5e/file-yzstVx8egG.gif)

### 3. Open "Templating" web services settings

Open the settings of the "Templating" plugin and scroll down to the web services by clicking on the small cog wheel on the right of the plugin in the list.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62979338a2c316231c201f57/file-z9Da5odwWW.gif)

### 4. Add your webservice

If your web service returns plain text (not a JSON), then you can just add it to the list of existing services using:

`"service name": "service url"` for example `"affirmation": "https://affirmations.dev"`.

If your service returns JSON like the one we are using in this example, you need to figure out the "path" to the piece of information you would like to have in your notes. It's easy if you are taking the current output and use a [JSON formatter](https://jsonformatter.curiousconcept.com/). In our example the path to the USD prices of bitcoin is `"bpi.USD.rate"`. For EUR prices it would be `"bpi.EUR.rate"`.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/62979508a2c316231c201f5b/file-YAFJ9ZEJtP.png)

Once you have figured this out add the service to the existing list in the settings which we opened before. In our example we need to add the following text:

```
 "bitcoin" : {
    "url" : "https://api.coindesk.com/v1/bpi/currentprice.json",
    "keys" : [
      "bpi.USD.rate"
    ]
  },
```

Let's add this to the settings and save them:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/629796a957320007925208ce/file-BYjyvhAFus.gif)

### 5. Use the web service in your template

We have named this web service "bitcoin". You can open an existing template or create a new one and then paste the following anywhere:

```
Bitcoin: <%- web.services('bitcoin') %> USD
```

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/629797c4a2c316231c201f76/file-lP0SF1X9nl.gif)

If you insert the template it will create the following output:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6297975ee1d2cf0eac00cf7f/file-F5nOGJ9CB6.gif)

---

We are done! If you have questions join our [Discord community](https://discord.gg/D4268MT) or reach out to [[email protected]](cdn-cgi/l/email-protection.md).

*Last updated: 2025-05-18*
