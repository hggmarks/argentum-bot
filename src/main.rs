use die_sir::evaluate;
use teloxide::{prelude::*, utils::command::BotCommands};
use std::error::Error;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Roll dice using standard RPG notation (e.g., /r 2d6+3)")]
    R(String),
    #[command(description = "Start the bot")]
    Start,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting Argentum dice rolling bot...");
    
    // Load environment variables from .env file
    dotenvy::dotenv()?;
    
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                "Hello! I'm Argentum, your dice rolling companion. Use /r followed by dice notation (e.g., /r 2d6+3) to roll dice, or /help to see all commands."
            ).await?;
        }
        Command::R(dice_expr) => {
            let response = match evaluate(dice_expr) {
                Ok(result) => format!("🎲 Result: {}", result),
                Err(e) => format!("❌ Error: {}", e),
            };
            bot.send_message(msg.chat.id, response).await?;
        }
    }
    Ok(())
}