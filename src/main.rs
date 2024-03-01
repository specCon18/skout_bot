use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready
    },
    prelude::*
};
use axum::{
    Router,
    routing::{get, post},
};
mod routes;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message is received - the
    // closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be dispatched
    // simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with('!') {
            return; // Not a command, so do nothing
        }
        // Sending a message can fail, due to a network error, an authentication error, or lack
        // of permissions to post in the channel, so log to stdout when some error happens,
        // with a description of it.
        let content: &str = msg.content.as_str();
        match content { 
            "!ping"=> if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            },
            "!pong" => if let Err(why) = msg.channel_id.say(&ctx.http, "Ping!").await {
                println!("Error sending message: {why:?}");
            },
            "!amethyst" => if let Err(why) = msg.channel_id.say(&ctx.http, "[Amethyst420's Gaming Channel](https://www.youtube.com/@Gaming_Amethyst420)").await {
                println!("Error sending message: {why:?}");
            },
            _=> if let Err(why) = msg.channel_id.say(&ctx.http, "I don't know that command.").await {
                println!("Error sending message: {why:?}");
            },
        }  
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Spawn the Discord bot in a separate async task
    let client_task = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {why:?}");
        }
    });

    // Setup Axum server
    let app = Router::new()
        .route("/webhooks/livestream", get(routes::stream_live))
        .route("/metrics", post(routes::metrics));

    let address = format!(
        "{}:{}",
        dotenv::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string()),
        dotenv::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string())
    );
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("rest endpoints @ {}", address);

    // Spawn the Axum server in a separate async task
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Await both tasks
    let _ = tokio::try_join!(client_task, server_task);
}
