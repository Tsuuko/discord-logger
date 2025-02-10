# Discord Logger

A simple tool that sends text received from standard input to a Discord Webhook.

>[!CAUTION]
> Some antivirus software might flag this tool as suspicious due to its network communication capabilities.
> This is a false positive - the source code is open and you can verify its safety.
> If you have concerns, please review the code before using the tool.

## Installation

From GitHub Release:
For Linux
```bash
curl -L https://github.com/Tsuuko/discord-logger/releases/latest/download/discord-logger -o discord-logger
chmod +x discord-logger
sudo mv discord-logger /usr/local/bin/
```

For Windows
Download discord-logger.exe from the latest release page and place it in your preferred location
https://github.com/Tsuuko/discord-logger/releases/latest

From GitHub with Cargo:
```bash
cargo install --git https://github.com/Tsuuko/discord-logger
```

From local:
```bash
cargo install --path .
```

## Usage

```bash
discord-logger <webhook-url>
```

Example:
```bash
echo "test message" | discord-logger https://discord.com/api/webhooks/your/webhook/url
```
