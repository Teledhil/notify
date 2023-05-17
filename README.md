# Notify

A library and console command for sending notifications to [Telegram
bots](https://core.telegram.org/bots/) and
[Firebase Cloud Messaging](https://firebase.google.com/docs/cloud-messaging)
projects.

## Setup

### Telegram credentials

* Use [BotFather](https://firebase.google.com/docs/cloud-messaging/js/client#configure_web_credentials_with)
  to obtain an authentication token for your bot.
* Get the identifier of a chat where your bot is. That chat will receive the
  messages sent with this tool. You can obtain the identifier by sending some
  message to that chat and then running:
```bash
./tools/get_chat_id.sh <your_authentication_token>
```

### Firebase Cloud Messaging credentials

* Create a Firebase projects [here](https://console.firebase.google.com/project/_/settings/cloudmessaging).
* Generate or import the credentials following [this
  steps](https://firebase.google.com/docs/cloud-messaging/js/client#configure_web_credentials_with).
* Pick a topic for the messages that will be sent. Only devices that have
  subscribed to that topic will receive the messages.

## Build

The required credentials are embedded into the notify-bin command. In order to
build it you need to create a `notify-bin/credentials.ini` file with the
Telegram and/or Firebase Cloud Messaging credentials:

```ini
[telegram]
token = "[TOKEN GOES HERE]"
chat_id = "[CHAT_ID GOES HERE]"

[firebase]
auth_key = "[SERVER KEY TOKEN GOES HERE]"
topic = "/topics/[TOPIC GOES HERE]"
```

Then you can build everything with cargo: `cargo build --release`.

## Usage

```bash
$ ./target/release/notify -h
notify-bin 0.1.0

USAGE:
    notify [OPTIONS] [TEXT]

ARGS:
    <TEXT>    Body of the message

OPTIONS:
    -h, --help                     Print help information
    -p, --photo <PHOTO>            Path to a picture
    -t, --title <TITLE>            Title of the message [default: hostname]
        --text-file <TEXT_FILE>    Path to a file. Contents of the file will be sent as text
    -V, --version                  Print version information
```

### Examples

* To send a simple message:
```bash
./target/release/notify "Hello World"
```
* To send a simple message with a custom title:
```bash
./target/release/notify --title "Announcement" "Hello World"
```
* To send the contents of a file as text:
```bash
./target/release/notify --title "Incoming Wall of Text" --text-file rumbling_thoughts.txt
```
* To send a picture, provide the path of the file:
```bash
./target/release/notify --photo image.jpg
```
* To send a picture with some caption:
```bash
./target/release/notify --photo cat.jpg "A 🐱"
```
