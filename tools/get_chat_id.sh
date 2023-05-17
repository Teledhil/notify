#!/bin/bash

bot_token=$1
endpoint="https://api.telegram.org/bot${bot_token}/getUpdates"

curl -s ${endpoint} | jq .result[0].message.chat.id
