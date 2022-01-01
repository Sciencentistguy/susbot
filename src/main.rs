use log::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use rand::seq::SliceRandom;

use clap::Parser;

const SUS_WORDS: &[&str] = &[
    "amogus",
    "among us",
    "amongus",
    "impostor",
    "impostors",
    "sus",
    "suspicious",
    "sussy",
    "vent",
];

// This is my bot's user ID. If you're someone else using this, you will have to change this
const BOT_MENTION_STR: &str = "<@!844330118364790815>";

static EMOJIS: Lazy<Vec<&'static str>> = Lazy::new(|| include_str!("emojis.txt").lines().collect());

struct Handler;

#[tokio::main]
async fn main() {
    // Set default log level to info unless otherwise specified.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "susbot=info");
    }

    pretty_env_logger::init();
    let opts = Opt::parse();
    let token = std::fs::read_to_string(opts.token_filename).expect("Failed to read token file");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = strip_md_chars(msg.content.to_lowercase().as_str());
        if content.contains("among us") {
            info!("Sus! author: {}; message: {}", msg.author.name, msg.content);
            let mut rng = rand::rngs::OsRng::default();
            msg.react(
                &ctx,
                serenity::utils::parse_emoji(EMOJIS.choose(&mut rng).unwrap()).unwrap(),
            )
            .await
            .expect("Failed to react to message");
        } else if msg.content.contains(BOT_MENTION_STR) {
            info!(
                "I was tagged. author: {}; message: {}",
                msg.author.name, msg.content
            );
            msg.react(&ctx, '👀')
                .await
                .expect("Failed to react to message");
        } else if content.split(' ').any(|x| SUS_WORDS.contains(&x)) {
            let mut num = 0;
            for word in SUS_WORDS.iter() {
                num += content.split(' ').filter(|x| x == word).count();
            }
            info!(
                "Sus! author: {}; message: {}; sus count: {}",
                msg.author.name, msg.content, num
            );
            let mut rng = rand::rngs::OsRng::default();
            for emoji in EMOJIS.choose_multiple(&mut rng, num) {
                msg.react(&ctx, serenity::utils::parse_emoji(emoji).unwrap())
                    .await
                    .expect("Failed to react to message");
            }
        }
    }
    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

/// sus
#[derive(Parser, Debug)]
#[clap(name = "susbot", version, author, about)]
struct Opt {
    /// File containing the bot token
    token_filename: PathBuf,
    /*
     * /// File containing the application id
     * application_id_filename: PathBuf,
     */
}

fn strip_md_chars(s: &str) -> String {
    s.chars()
        .filter(|c| {
            !matches!(
                c,
                '*' | '_'
                    | '~'
                    | '|'
                    | '`'
                    | '.'
                    | '?'
                    | '"'
                    | '‘'
                    | ','
                    | '-'
                    | '—'
                    | '!'
                    | ':'
                    | ';'
                    | '('
                    | ')'
                    | '['
                    | ']'
                    | '…'
                    | '/'
                    | '{'
                    | '}'
            )
        })
        .collect()
}
