# Travel Checklist

Traveling is stressful, and if you are traveling often you face the same question every time: "Did I forget anything?". It's just too easy to forget something. Instead of trying to remember what you need to take or do, let technology do the work and make your travels more relaxing with a checklist template.

The following template (see bottom of the article) will give you a list of all the important things. Remove, change or add things that are missing. When you insert the template it will also ask you what the destination is, and how many days you travel.

You can add more questions if you want to document your travels further, such as the purpose of the trip, who are the other traveling companions, etc. by using the prompt template tag. For example, the following tag will ask you "How many days?" and write the response into your note:

```
<%- prompt('How many days?') %>
```

## **![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6294f773a2c316231c2013d3/file-tWFIVoaLrj.png)
**

## **How to add the template**

Copy the text below and paste it into a new note in your "Templates" folder (under "Smart Folders"):

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6294ee5202b8fd3b945f89e1/file-F94pbRJ2Qz.gif)

## How to use the template

Create a new note in any folder and click/tap on the "Insert Template" button that you can see inside empty notes:

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6294eeff92cb8c175b468278/file-PVJhxLdANK.gif)

## Template

```
---
title: Travel Checklist
type: empty-note 
---
# ✈️ Travel Checklist - <%- prompt('Where are we going?') %> (<%- date.now("MMMM 'YY") %>)
*Date:* <%- date.now("Do MMM YYYY") %>
*Days*: <%- prompt('How many days?') %>

**🛂 Travel Requirements**
* Online Flight Check-in (24h before)
	> Tip: Save the boarding ticket to the daily note of the flight
* Check and fill online registration (as required in the destination country)
* Check and plan for health requirements (such as for COVID)

**📄 Documents**
* Boarding pass
* Passport / ID Cards
* Hotel reservation
* Rental car reservation
* Drivers license
* Insurance card
* Credit cards

**💼 Packing**
- Electronics
	* Laptop
	* Tablet
	* Phone
	* Headphones
	* Charging equipment
	* SIM cards
- Weather: Sunny? Cold?
	* Sunblocker
	* Cap
	* Jacket
	* Umbrella
- Activities
	* Swimming clothes
	* Sport shoes
	* Gym/running clothes
```

*Last updated: 2022-05-30*
