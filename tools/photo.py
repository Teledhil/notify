#!/usr/bin/env python3

from __future__ import absolute_import
from __future__ import division
from __future__ import print_function

import argparse
import asyncio
import io
import json
import requests
import sys
import time

# Telegram token and chat_id
credentials = [ ("__TOKEN__", "__CHAT_ID__")]

async def notify(token, chat_id, photo):
    await notify_telegram_photo(token, chat_id, photo)

async def notify_telegram_photo(token, chat_id, photo):

    NOTIFY_URL = "http://127.0.0.1:8000/bot" + token + "/sendPhoto"

    message = "🦁"

    translations = {
            ord("."): "\.",
            ord("-"): "\-",
            ord("["): "\[",
            ord("]"): "\]",
            ord("("): "\(",
            ord(")"): "\)",
            ord("%"): "\%",
            }
    message = message.translate(translations)

    files = {'photo': open(photo, 'rb')}

    data = {
            "chat_id": chat_id,
            "caption": message,
            "parse_mode": "MarkdownV2",
            }

    r = requests.post(NOTIFY_URL, data=data, files=files)
    print(r)


async def main():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument(
        '--image',
        help='File path of image file.',
        required=True)

    args = parser.parse_args()

    for token, chat_id in credentials:
        await notify(token, chat_id, args.image)

if __name__ == "__main__":
  asyncio.run(main())
