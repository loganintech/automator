import math
import random
import discord
import asyncio
from pathlib import Path
import os

global status
status = False
global currchannel
currchannel = ""


class DClient(discord.Client):
    async def on_message(self, message):
        if math.floor(random.random()) % 20 != 0:
            return
        global status
        global currchannel
        for guild in self.guilds:
            for channel in guild.channels:
                if isinstance(channel, discord.VoiceChannel):
                    members = channel.members
                    if "JewsOfHazard#0001" in [str(member) for member in members]:
                        if not status or currchannel != channel:
                            with open(os.path.join(Path.home(), "Dev", ".is_in_discord"), "w+") as f:
                                f.write(str(channel))
                        status = True
                        return

        with open(os.path.join(Path.home(), "Dev", ".is_in_discord"), "w+") as f:
            f.write("")
        status = False


client = DClient()
with open("token.txt", "r") as f:
    token = f.readline()
    client.run(token, bot=False)
