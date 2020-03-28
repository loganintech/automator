import math
import random
import discord
import asyncio
from pathlib import Path
import os


class DClient(discord.Client):

    currchannel = ""
    status = False
    filepath = os.path.join(Path.home(), ".is_in_discord")

    async def on_message(self, message):
        if math.floor(random.random()) % 20 != 0:
            return
        for guild in self.guilds:
            for channel in guild.channels:
                if not isinstance(channel, discord.VoiceChannel):
                    continue
                members = channel.members
                if "JewsOfHazard#0001" in [str(member) for member in members]:
                    if not self.status or self.currchannel != channel:
                        with open(self.filepath, "w+") as f:
                            f.write(str(channel) + "\n")
                    self.status = True
                    return

        with open(self.filepath, "w+") as f:
            f.write("")
        self.status = False


client = DClient()
with open("token.txt", "r") as f:
    token = f.readline()
    client.run(token, bot=False)
