# Import Libs
import discord, time, datetime
from discord.ext import commands
from config import rich_message, server_id, reaction_roles

# Create extension class
class Main(commands.Cog, name='Main'):
    # COG init
    def __init__(self, bot):
        self.bot = bot

    # Payload function
    async def reaction(self, payload, r_type=None):
        if payload.guild_id == server_id: 
            for reaction_info in reaction_roles: 
                if payload.message_id == reaction_info[0]: 
                    if payload.emoji.name == reaction_info[2]: 
                        guild = self.bot.get_guild(payload.guild_id)
                        user = guild.get_member(payload.user_id)
                        role = discord.utils.get(guild.roles, name=reaction_info[1])
                        if r_type=='remove': await user.remove_roles(role)
                        if r_type=='add': await user.add_roles(role)

    # Create on reaction add:
    @commands.Cog.listener()
    async def on_raw_reaction_add(self, payload):
        await Main.reaction(self, payload=payload, r_type='add')

    # Create on reaction remove:
    @commands.Cog.listener()
    async def on_raw_reaction_remove(self, payload): 
        await Main.reaction(self, payload=payload, r_type='remove')

    # When the extension is loaded
    @commands.Cog.listener()
    async def on_ready(self):
        await self.bot.change_presence(status=discord.Status.online, activity=discord.Game(name=rich_message, url='https://www.twitch.tv/bel_justice'))
        print('Successfully loaded the \'Main\' extension.')


# Extension setup: 
def setup(bot):
    print('Started loading the \'Main\' extension.')
    m = Main(bot)
    bot.add_cog(m)

