reaction_logger = {
    "added": {
        "title": "Emoji added:",
        "content": "User: {member.mention} (`{member.id}`)\n"
                   "Channel: {channel.mention} (`{channel.id}`)\n"
                   "Emoji: {emoji.name}\n",
        "footer": {
            "text": "Emoji Logger",
            "icon": "{guild.icon_url}",
            "timestamp": True
        },
        "color": {
            "random": False,
            "color": 0x00ff00
        }
    },
    "removed": {
        "title": "Emoji removed:",
        "content": "User: {member.mention} (`{member.id}`)\n"
                   "Channel: {channel.mention} (`{channel.id}`)\n"
                   "Emoji: {emoji.name}\n",
        "footer": {
            "text": "Emoji Logger",
            "icon": "{guild.icon_url}",
            "timestamp": True
        },
        "color": {
            "random": False,
            "color": 0xff0000
        }
    }
}
