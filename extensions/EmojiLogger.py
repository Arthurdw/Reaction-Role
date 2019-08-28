# Import Libs
import discord, time, datetime
from discord.ext import commands
from config import rich_message, server_id, logging_channel, date_format

message_types = {
    'add': ("", "{c}", "", 0x00ff00),
    'remove': ("", "{c}", "", 0xff0000)
    }

def embed_message(content=None, title=None, type=None, icon_url=None):
    emb_title, content_format, icon, color = message_types.get(type) or message_types.get(None)
    title = title or emb_title
    embed = discord.Embed(color=discord.Color(color), description=content_format.format(c=content))
    embed.set_author(name=title, icon_url=icon)
    embed.set_footer(text='Emoji Logger', icon_url=icon_url)
    return {"embed": embed}

class EmojiLogger(commands.Cog, name='EmojiLogger'):
    def __init__(self, bot):
        self.bot = bot

    async def reaction(self, payload, r_type=None):
        guild = self.bot.get_guild(server_id)
        channel = guild.get_channel(payload.guild_id)
        member = guild.get_member(payload.user_id)
        logs = guild.get_channel(logging_channel)
        if r_type == "add": await logs.send(embed_message(title="Emoji added:", type='add', content=f'User: {member.mention} (`{member.id}`)\nChannel: {channel.mention}\nEmoji: {payload.emoji.name}\nDate: {datetime.datetime.now().strftime(date_format)}', icon_url=guild.icon_url))
        elif r_type == "remove": await logs.send(embed_message(title="Emoji removed:", type='remove', content=f'User: {member.mention} (`{member.id}`)\nChannel: {channel.mention}\nEmoji: {payload.emoji.name}\nDate: {datetime.datetime.now().strftime(date_format)}', icon_url=guild.icon_url))

    @commands.Cog.listener()
    async def on_raw_reaction_add(self, payload):
        await EmojiLogger.reaction(self, payload=payload, r_type='add')

    @commands.Cog.listener()
    async def on_raw_reaction_remove(self, payload): 
        await EmojiLogger.reaction(self, payload=payload, r_type='remove')

    @commands.Cog.listener()
    async def on_ready(self):
        await self.bot.change_presence(status=discord.Status.online, activity=discord.Game(name=rich_message, url='https://www.twitch.tv/bel_justice'))
        print('Successfully loaded the \'EmojiLogger\' extension.')

def setup(bot):
    print('Started loading the \'EmojiLogger\' extension.')
    m = EmojiLogger(bot)
    bot.add_cog(m)

