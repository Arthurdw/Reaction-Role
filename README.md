# Discord Reaction Role Bot

###### Using UtilsX (w discord.py)

![Signing In Example](https://the-earth.is-inside.me/gSC4Nmdo.gif)

## Requirements

To be able to run this bot you have [Python](https://www.python.org). (Min: [3.7.x](https://www.python.org/downloads/release/python-379/), Max: [3.8.x](https://www.python.org/downloads/release/python-386/))  
When Python has been installed open your CLI and navigate to the directory.
And when you are in the correct directory execute the following command to install the dependencies:
```
(python3 -m | py -3 -m) pip install -r requirements.txt
```

## Troubleshooting

If you are getting a `ImportError` please re-run the install command.
If this doesn't fix the problem please run the following command:
```
(python3 -m | py -3 -m) pip install -U discord.py
```

## Setup

This bot is very easy to setup and only requires a few steps!  
For any discord application you need a Discord Bot token.  
You can get your own bot token from the [Discord Developer Application site](https://discordapp.com/developers/applications/me).  
Or follow [this](https://github.com/Arthurdw/Reaction-Role/wiki/How-to-create-your-bot-and-find-your-own-bot-token!) tutorial.

Once you have your bot token you can head over to the `config` folder and open the `config.cfg` file.
At the bottom of that file you will see `token = XXXYOURBOTTOKENHEREXXX`, 
replace the `XXXYOURBOTTOKENHEREXXX` with your discord bot token.

And voila, your bot should start. But the reaction roles will not work yet.  
For the reaction roles to be able to work as we wish we need to configure them first.

### Configuring the reaction roles

To add our own reaction roles we can head over to the `config` folder and open the `reaction_roles.py` file.
In there you will see the following content:

```py
reaction_roles = {
    123456789987654321: [
        ("😃", 123456789987654321)
    ]
}
```

Lets go over each line so you know exactly what it does and you can configure your bot properly.  

```py
reaction_roles = { <--
   ...
} <--
```

These two lines tell our program that the content between those lines are our reaction roles.  
**Never remove these!**

```py
reaction_roles = {
    123456789987654321: [ <--
        ...
    ] <--
}
```

This tells our program what the message id should be for the reaction role(s).  
Within those two brackets will be the roles and their emojis.   
You can add as many messages like this as you wish, as long as these are delimited by a `,`.  
For example:

```py
reaction_roles = {
    123456789987654321: [ ... ], <-- First Message
    234567899876543200: [ ... ]  <-- Second Message
}
```

To add a reaction role just place the following data between the brackets of your message.  
For example:

```py 
reaction_roles = {
    123456789987654321: [
        ("😃", 123456789987654321),
        ("emoji", role_id)
    ]
}
```

As you can see the same delimiter rule applies to the reaction roles.

### Configuring the reaction logger

If you would like to log all emoji's, you can easily enable this function in the `config.cfg` *(in the `config` folder)* file 
and head over to the `REACTION_LOGGING` section.

Once you are there you only need to set `enabled` to `true` (`enabled = true`)
But to be able to make it send the logs to the right place change the `log_guild` value to 
your guild its id *(discord server)* and `log_channel` to your channel its id.

Then reboot the bot and everything should work!

## Running the bot

To run the bot simply run the following command:
```
(python3 | py -3) run.py
```

## Creating custom extensions

This bot is fully optimized to be able to get expanded with ease. Just place your extension file in 
the extensions folder and reboot the bot. The bot will automatically try to load that extension.  
If you are creating a fresh extension for this bot we recommend that you use the [UtilsX](http://docs.xiler.net/utilsx) 
library as it makes writing discord.py code easier and faster.

## Having issues?
Please [create an issue](https://github.com/Arthurdw/Reaction-Role/issues/new) or join [our discord](https://discord.gg/Z6dw5pw) and I'll help you out!

## Support this project

You are always free to donate me though PayPal.  
[paypal.me/ArthurDeWitte](http://paypal.me/ArthurDeWitte)

### Special thanks

Special thanks to `CombatMedic02#2610` for supporting me in this project ♥
