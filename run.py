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
from configparser import ConfigParser
from datetime import datetime
from distutils.util import strtobool
from glob import glob
from os import name, execv, system, environ
from sys import argv, executable, stdout, exit

from discord import LoginFailure, Game, Streaming, Status
from utilsx.console import Prettier, Colors
from utilsx.discord import BotX

from utils import VersionHandler, PrintHandler

# Check if the operating system is linux or windows. (nt = windows)
# If its windows, change the console clear command and the filepath delimiter.
clear, back_slash = "clear", "/"
if name == "nt":
    clear, back_slash = "cls", "\\"

# Read our configuration
cfg = ConfigParser()
cfg.read("./config/config.cfg")


class Bot(BotX):
    """
    The main bot object, this contains our handlers and loads our extensions
    """

    def __init__(self, _p: Prettier, _ph: PrintHandler):
        super().__init__()
        system(clear)
        stdout.flush()
        self.prettier = _p
        self.ph = _ph
        self.ph.info("Initializing client...")
        self.prefix = cfg["BOT"].get("prefix", "!")

        self.vm = VersionHandler()
        if strtobool(cfg["UPDATER"].get("enabled", "true")):
            self.check_for_updates()
            self.ph.info("No updates found, starting bot...")

        self.ph.info("Started loading extensions.")
        extensions = list(map(lambda extension: extension.replace(back_slash, ".")[:-3], glob("extensions/*.py")))

        for index, _ in enumerate(self.load_extensions(extensions)):
            if strtobool(cfg["CONSOLE"].get("print_imports", "true")):
                self.ph.info(f"Successfully loaded "
                             f"{Colors.light_blue.value + extensions[index].replace('extensions.', '')}")

    @staticmethod
    def restart():
        system(clear)
        stdout.flush()
        execv(executable, ['python'] + argv)

    def check_for_updates(self):
        self.ph.info("Checking for updates...")
        if not self.vm.is_latest:
            self.ph.info("Update found! Started updating bot to the latest version...")
            self.vm.update_version()
            self.ph.info("Successfully updated to the latest version. Rebooting bot.")
            self.restart()

    async def on_ready(self):
        self.ph.info(f"Currently running on v{self.vm.version}!")

        async def get_correct_bot_object(path: str, objects: list):
            value = cfg["BOT"].get(path, None)
            if not value:
                self.ph.warn("Presence path could not be found!")
                self.ph.fatal("Invalid config file.")
                raise SystemExit(1)
            for obj in objects:
                if obj[0] == value.lower():
                    return obj[1]
            self.ph.warn(f"Could not parse given configuration for value '{path}' ['{value}' was given]")
            self.ph.fatal("Invalid config file.")
            raise SystemExit(1)

        statuses = [("online", Status.online), ("dnd", Status.dnd), ("idle", Status.idle),
                    ("invisible", Status.invisible)]

        try:
            if strtobool(cfg["BOT"].get("rich_presence_enabled")):
                activities = [("playing", Game), ("streaming", Streaming)]
                await self.change_presence(
                    activity=(await get_correct_bot_object("rich_presence_type", activities))(
                        name=cfg["BOT"].get("rich_presence", "Invalid configuration"),
                        start=datetime.now(),
                        url="https://www.twitch.tv/bel_justice"),
                    status=await get_correct_bot_object("bot_status", statuses)
                )
            else:
                await self.change_presence(status=await get_correct_bot_object("bot_status", statuses))
        except SystemExit as err:
            await self.close()
            raise err


if __name__ == "__main__":
    prettier = Prettier(colors_enabled=strtobool(cfg["CONSOLE"].get("colors", "true")),
                        auto_strip_message=True)
    ph = PrintHandler(prettier, strtobool(cfg["CONSOLE"].get("print_log", "true")))
    token = None

    if strtobool(cfg["TOKEN"].get("token_env_enabled", "false")):
        location = cfg["TOKEN"].get("token_env", "RR_BOT_TOKEN")
        try:
            token = environ[location]
        except KeyError as e:
            ph.fatal("The requested environment variable doesn't exist.\n"
                     "Please check if you have reopened your CLI or of it is set.")
            exit(1)
    else:
        token = cfg["TOKEN"]["token"]

    try:
        Bot(prettier, ph).run(token)
    except LoginFailure:
        ph.fatal("A wrong bot token was provided!")
    except SystemExit:
        ph.fatal("Shutting down bot, caused by previous warning or fatal.")
    except Exception as e:
        ph.fatal(f"Shutting down bot, error:\n{e}")
