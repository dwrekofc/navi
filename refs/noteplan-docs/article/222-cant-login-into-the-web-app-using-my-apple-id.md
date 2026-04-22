# Can't login into the web app using my Apple ID

There can be various reasons why the Apple login is not working, please see the checklist below:

1. Make sure you **don't** have **"Advanced Data Protection"** enabled on macOS or iOS in your iCloud settings. Unfortunately, Apple blocks logging in and accessing your iCloud data over a website if this is turned on.
2. Disable browser extensions. You can also open an Incognito/Private window first (where the extensions are not loaded) and try there.
3. Some browsers are blocking third-party cookies and the Apple login requires setting cookies for the login to complete. You need to enable third-party cookies or add an exception for "app.noteplan.co". See below for instructions.
4. In some environments CloudKit (from Apple) might be blocked. Allow access for "https://api.apple-cloudkit.com", "https://app.noteplan.co" in your firewall or VPN settings like "ZSCALER". You might need to bypass the SSL inspection for these URLs.
5. If you can login, but see a trial banner at the top (though you are subscribed), whitelist following websites (or the full domain) in your blocker, if you are using any with a VPN or other services that might block sites: "https://stripe.com", "https://r.stripe.com", or domain "stripe.com", and "https://api.noteplan.co".

## Cookies Chrome

1. Open the Settings.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66195eca5b008e1c56a993ec/file-Ax3hL9zQ8v.png)2. Search for "Cookies" and select "Third-Party Cookies".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66195f0dd9cffa6c1049d4c1/file-az2tZIrLwM.png)3. Scroll down to "Customized behaviors" and click on "Add" for "Allowed to use third-party cookies".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66195f6a477c990280e43804/file-qaAaKZgXJ3.png)4. Add "app.noteplan.co".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66195f95ef6cd1080a2eae20/file-O4Q7qu0fku.png)Go back to [NotePlans web app](https://app.noteplan.co), reload the page and try to login with your Apple ID.

## Cookies Arc Browser

1. Open the Settings.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/661864b1477c990280e43760/file-alABXio8pK.png)

2. Click on "Advanced", then "More Settings".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/661864c84db6eb7a51d9367f/file-Gftdgs3CRt.png)3. Click on the search bar at the top and search for "Cookies". Select "Third-Party Cookies".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66195f0dd9cffa6c1049d4c1/file-az2tZIrLwM.png)

4. At the bottom click on "Add" for "Allowed to use third-party cookies" and type "app.noteplan.co".

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/66186529dac5c26585fc63a0/file-zL9i9G4HoD.png)

## Cookies Safari

Open the settings and under "Advanced" turn off "Block all cookies":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/661960ae477c990280e43806/file-4aBqYv3UyU.png)

*Last updated: 2025-09-27*
