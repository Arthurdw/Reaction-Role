# Reaction Role Bot
### Using Discord.py Rewrite!

## Setup:
This bot is very easy to setup and just requires a few steps.  
First of all you need to get your discord bot token. ("`token = `")  
You can retrieve this from the [Discord Developer Application site](https://discordapp.com/developers/applications/me 'Discord Developer Platform')!    
Or follow [this](https://github.com/Arthurdw/UwU/wiki/How-to-create-your-bot-and-find-your-own-bot-token! 'Find your bot token!') tutorial!  
  
Then fill in your server id, click [here](https://support.discordapp.com/hc/en-us/articles/206346498-Where-can-I-find-my-User-Server-Message-ID- "Discord's server ID tutorial!") ("`server_id =`")  
And finally add your reaction roles.
> ```python
> # Single reaction role instance:
>                   # Message ID        # Role Name  # Emoji
> reaction_roles = [(114034353626021889, 'role_name', '😃')]
> ```  
Or if you want more roles with more reactions you can use it this way:
> ```python
> # Multiple reaction role instance:
>                   # Message ID        # Role Name  # Emoji
> reaction_roles = [(114034353626021889, 'role_name', '😃'),
>                   # Second Message ID  # Second Role      # Second Emoji
>                   (114034353626021889, 'second_role_name', '😅')]
> ```
So you can create an infinite amount of reaction roles!  
  
The rich message, changes the bot its presence! ("`rich_message =`")  
The message that is assigned to the variable will be displayed like this:  
![First](https://gyazo.com/9ab8ecfbb0967b7aee313d2fe1638670 "Rich Presence")  
![Second](https://gyazo.com/ecc60c3b0ca08e8d9ebc0ec53e6c72db "Rich Presence")  


## Using this as an extension:
If you would like to use this as an extension for your bot, you are free to do so.
Just use the '`Main`' file from the '`extensions`' folder!
