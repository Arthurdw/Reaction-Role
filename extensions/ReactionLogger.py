# -*- coding: utf-8 -*-
"""
MIT License

Copyright (c) 2019-2020 Arthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
from distutils.util import strtobool

from discord import RawReactionActionEvent
from utilsx.discord import Cog
from utilsx.discord.objects import Footer

from config.lang import reaction_logger
from run import cfg


class ReactionLogger(Cog):
    def __init__(self, bot):
        super().__init__()
        self.bot = bot
        self.channel = None

    @Cog.listener()
    async def on_ready(self):
        guild = self.bot.get_guild(int(cfg["REACTION_LOGGING"]["log_guild"]))
        self.channel = guild.get_channel(int(cfg["REACTION_LOGGING"]["log_channel"]))

    def channel_is_set(self):
        if not self.channel:
            self.bot.ph.warn("No valid emoji log channel has been found yet, ignoring log.")
            return False
        return True

    @Cog.listener()
    async def on_raw_reaction_add(self, payload: RawReactionActionEvent):
        if not self.channel_is_set():
            return
        guild = self.bot.get_guild(payload.guild_id)
        member = await guild.fetch_member(payload.user_id)
        channel = guild.get_channel(payload.channel_id)
        await self.embed(self.channel, title=reaction_logger["added"]["title"],
                         message=reaction_logger["added"]["content"].format(member=member, guild=guild,
                                                                            emoji=payload.emoji, channel=channel),
                         footer=Footer(reaction_logger["added"]["footer"].get("text"),
                                       reaction_logger["added"]["footer"].get("icon").format(guild=guild),
                                       reaction_logger["added"]["footer"].get("timestamp")),
                         color=(None if reaction_logger["added"]["color"].get("random", True) else
                                reaction_logger["added"]["color"].get("color", 0x00ff00)))

    @Cog.listener()
    async def on_raw_reaction_remove(self, payload: RawReactionActionEvent):
        if not self.channel_is_set():
            return
        guild = self.bot.get_guild(payload.guild_id)
        member = await guild.fetch_member(payload.user_id)
        channel = guild.get_channel(payload.channel_id)
        await self.embed(self.channel, title=reaction_logger["removed"]["title"],
                         message=reaction_logger["removed"]["content"].format(member=member, guild=guild,
                                                                              emoji=payload.emoji, channel=channel),
                         footer=Footer(reaction_logger["removed"]["footer"].get("text"),
                                       reaction_logger["removed"]["footer"].get("icon").format(guild=guild),
                                       reaction_logger["removed"]["footer"].get("timestamp")),
                         color=(None if reaction_logger["removed"]["color"].get("random", True) else
                                reaction_logger["removed"]["color"].get("color", 0xff0000)))


def setup(bot):
    if strtobool(cfg["REACTION_LOGGING"].get("enabled", "true")):
        bot.add_cog(ReactionLogger(bot))
