# Discord Reaction Role Bot

![Made with](https://img.shields.io/badge/Made%20With-LOVE-%23fa4b4b?style=flat-square)
![MIT License](https://img.shields.io/github/license/Arthurdw/Reaction-Role?style=flat-square)
![Language](https://img.shields.io/badge/Language-Rust-%23B7410E?style=flat-square)

## About

> 🔥 The bot has been fully rewritten in Rust! 🔥
> _(check out [Migrating](#migrating-from-the-python-version))_

This is a easy to use Discord bot with human friendly configuration files.

### Features

- Emoij Reaction Roles (also works with custom emoji's)
- Experience/Leveling system with leaderboard
- Token configurable from env (also loads .env) or config file
- Reaction logging
- Chat and slash command support (configurable)
- Configurable rich presence
- Configurable verbose logging

## Getting started

### Discord setup

TODO: write this section with images etc

### Configuring the bot

TODO: write this section with images etc

### Running the bot (Docker)

Now that you have your bot token, you can use the provided docker compose file
to easily get your bot up and running.

In the project directory _(where the `docker-compose.yml` file is located)_, run
the following command:

```bash
docker compose up
```

If you have just pulled an updated version of the bot, you might want to rebuild
the image:

```bash
docker compose up --build
```

#### Running in the background

To run the bot in the background, you can use the `-d` flag:

```bash
docker compose up -d
```

### Running from source

This bot is written in Rust, so you will need to have Rust installed on your
system. Openssl is also required for the bot to work. If you are using the
Nix package manager, you can load the `flake.nix` file.

Once all dependencies are installed, you can run the bot with the following

```bash
cargo run --release
```

## Migrating from the Python version

**BEFORE MIGRATING, MAKE SURE TO TURN OFF THE BOT AND BACKUP YOUR `db` FOLDER IF
PRESENT!**

The only change required is in the config.
Make the `.py` and `.cfg` files to the expected YAML format.
All data will keep working.

### Why moving to Rust?

Rust provides a more stable development environment and is just my preferred
language. As an added bonus, it's faster and more efficient than Python.

### Why moving to YAML?

The YAML format is language independent and is easier to read and write.
This change allows for more flexibility in the configuration and for a
potentional future where the bot is rewritten in another language.

## Having issues?

Please [create an issue](https://github.com/Arthurdw/Reaction-Role/issues/new)
or join [our discord](https://dc.arthurdw.com) and I'll help you out!

## Support this project

You are always free to donate me though PayPal.  
[paypal.me/ArthurDeWitte](http://paypal.me/ArthurDeWitte)

### Special thanks

Special thanks to `combatmedic02` for supporting me in this project ♥
