# Argentum Bot 🎲

A Telegram bot for RPG dice rolling, built with Rust.

## Features

- Roll dice using standard RPG notation (e.g., `2d6+3`)
- Support for multiple dice rolls
- Clean and simple interface
- Fast and reliable results

## Setup

1. Install Rust if you haven't already: https://rustup.rs/
2. Clone this repository
3. Create a new bot and get your token from [@BotFather](https://t.me/botfather) on Telegram
4. Copy `.env.example` to `.env` and add your bot token:
   ```
   TELOXIDE_TOKEN=your_telegram_bot_token_here
   ```
5. Build and run the bot:
   ```bash
   cargo run
   ```

## Usage

Once the bot is running, you can use these commands in Telegram:

- `/start` - Initialize the bot
- `/help` - Show available commands
- `/roll <expression>` - Roll dice (e.g., `/roll 2d6+3`)

## Examples

- `/roll d20` - Roll a 20-sided die
- `/roll 2d6+3` - Roll two 6-sided dice and add 3
- `/roll 4d6` - Roll four 6-sided dice

## Development Status

🐾🐾 Under Development 🐾🐾