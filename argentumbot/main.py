import logging

from telegram.ext import CommandHandler, Updater

import botcommands.commands
import settings

logging.basicConfig(
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    level=logging.INFO)

updater = Updater(token=settings.API_TOKEN, use_context=True)
dispatcher = updater.dispatcher

frase_handler = CommandHandler('diga', botcommands.commands.frase)
dispatcher.add_handler(frase_handler)

updater.start_polling()
