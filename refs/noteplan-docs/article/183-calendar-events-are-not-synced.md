# Calendar events are not syncing or sync is slow

NotePlan uses Apple's calendar to sync events. You can [add calendars to your system account ](28-how-to-add-new-calendars.md)and NotePlan will gain access automatically. But sometimes the sync between your devices might not be as fast as you expect it to or you have to open the iCal app to trigger the sync. If you are using Google, Outlook, or other non-iCloud calendars, the sync might be slower.

You can check the following steps to resolve the issue. Before you start make sure you have restarted all the affected devices just to make sure this is causing the problem.

## 1. Review refresh preferences

You can review your calendar event refresh settings. "Push" should be the fastest sync as events are pushed directly to the devices as soon as you make a change. You might not be able to select "Push" for calendars like Google, but you can decrease the fetch time to a minute on Mac and 15min minimum on iOS.

### On Mac

Open the preferences of Apple's default calendar app "Calendar", then "Account":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63e3ac42f19b3c454741d7fe/file-uDmbdiRVzw.png)

Reduce the fetch time for non-iCloud calendars such as Google. The default is 15min:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63e3ae72f19b3c454741d803/file-Rd1aRFq4zQ.png)

### On iOS

Open the settings app, then type "Calendar" into the search and select Apple's calendar app settings:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63e3ad75d4571976c49de9e0/file-1pDWqrG0Oi.png)

Then select "Accounts" > "Fetch New Data". Make sure "Push" is enabled at the top and selected for "iCloud".

## 2. Reset calendars by turning them off/on

Using the same preferences mentioned in the first step or under "Internet Accounts", you can disable and enable the calendar option from your various accounts. If you are using iCloud and Google, for example, try disabling Google too, even if you don't have problems with this calendar in specific.

### On Mac

1. Open "Internet Accounts" from your system preferences (or search Spotlight for "Internet Accounts"), then select an account.
2. Click on the switch to turn the calendar off and then enable it right away.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63e4f5acf19b3c454741d98e/file-Bl7VYYteby.png)

### On iOS

Open the Settings app and then search for "Calendar" > "Accounts" > Select your account (like Gmail) > Turn off/on "Calendars":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/63e4f704f5e6e535e1a96751/file-7RWERHE0dA.png)

*Last updated: 2023-02-09*
