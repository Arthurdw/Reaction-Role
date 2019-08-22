# Import required library's or files
import discord as dc
from discord.ext import commands as cmd
from config import token

# Create list with extensions in it.
EXTENSIONS = ['Main']

# Create Class for the bot to start in.
class Reaction(cmd.Bot):
    # Bot init setup
    def __init__(self):
        prefixes = ['!']
        super().__init__(command_prefix=cmd.when_mentioned_or(*prefixes),
                         description='Reaction',
                         help_attrs=dict(hidden=True))
        self.token = token
        self.remove_command('help')

        # Select an item in the list we created and try to load it.
        for extension in EXTENSIONS:
            try:
                full = 'extensions.' + extension
                self.load_extension(full)
            except Exception as e: print('An error occurred while trying to load the "{}" extension.\n{}: {}'.format(extension,
                                                                                                  type(e).__name__, e))
            
    # Setup message when the bot starts!
    async def on_ready(self):
        print(f'------------\nLogged in as:\nUsername: {self.user.name}\nID: {self.user.id}\n------------')

    #check if the message author is a bot & process the message. (check if a command is in it ect)
    async def on_message(self, message):
        if message.author.bot: return
        await self.process_commands(message)

    # run the bot
    def run(self):
        super().run(self.token, reconnect=True)


if __name__ == '__main__':
    bot_Reaction = Reaction()
    bot_Reaction.run()