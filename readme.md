# Susbot

Sus.

![sus](https://cdn1.iconfinder.com/data/icons/logos-brands-in-colors/231/among-us-player-red-512.png "sus")

## Usage

- [Create](https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account) a discord application and bot.
- [Invite](https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot) the bot to your server.
- Enable the `MESSAGE_CONTENT` privileged intent
- Create two files, containing the bot token and application id
- Run the bot, providing the token and application id as command line arguments:
  - With the included NixOS module:
  ```nix
  {pkgs, ...}: {
    services.bonkbot = {
        enable = true;
        tokenFile = "<token_filename>";
        appIdFile = "<application_id_filename>";
    };
  }
  ```
  - With nix:
    - `nix run 'github:Sciencentistguy/susbot' -- <token_filename> <application_id_filename>`
  - With cargo:
    - `cargo run -- <token_filename> <application_id_filename>`
  - Standalone (you probably need to install it on your system first):
    - `susbot <token_filename> <application_id_filename>`

---

Written using [Serenity](https://github.com/serenity-rs/serenity).

Available under the terms of the Mozilla Public Licence, version 2.0
