import urllib
import urllib.request

from telegram import Update
from telegram.ext import CallbackContext

import botcommands.replies


def frase(update: Update, Context: CallbackContext):
    bot_message = botcommands.replies.answer.get(
        update.message.text[6:].lower())
    update.message.reply_text(f'{bot_message}')
