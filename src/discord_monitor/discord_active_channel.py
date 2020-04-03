import math
import random
import discord
import asyncio
from pathlib import Path
import os
import setproctitle
import sys

setproctitle.setproctitle("automator-discord_active_channel")


class DClient(discord.Client):

    currchannel = ""
    is_in_channel = False
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
                    if not self.is_in_channel or self.currchannel != channel:
                        with open(self.filepath, "w+") as f:
                            f.write(str(channel) + "\n")
                    self.is_in_channel = True
                    return

        with open(self.filepath, "w+") as f:
            f.write("")
        self.is_in_channel = False

    async def on_error(self, _args, _kwargs):
        pass


async def main():

    while True:
        try:
            client = DClient()
            with open("token.txt", "r") as f:
                token = f.readline()
                await client.login(token, bot=False)
                await client.connect(reconnect=False)
        except Exception as e:
            print("Error occurred running the bot:", e, file=sys.stderr)

if __name__ == "__main__":
    asyncio.run(main())
