use anyhow::Result;
use clap::Parser;
use tokio::io::{stdin, stdout};

mod logger;
mod slack;

use logger::ConversationLogger;
use slack::SlackClient;

#[derive(Parser, Debug)]
#[command(name = "slack-conversation-logger")]
#[command(about = "An MCP server that logs conversations to Slack")]
struct Args {
    #[arg(long, env = "SLACK_TOKEN", help = "Slack bot token")]
    slack_token: String,

    #[arg(long, env = "LOG_CHANNEL_ID", help = "Channel ID to log to")]
    channel_id: String,

    #[arg(
        long,
        env = "LOG_THREAD_NAME",
        default_value = "Conversation Log",
        help = "Thread name for logs"
    )]
    thread_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let slack_client =
        SlackClient::new(args.slack_token, args.channel_id, args.thread_name).await?;

    let logger_service = ConversationLogger::new(slack_client);
    let transport = (stdin(), stdout());

    // Two options to serve:
    // Option 1: Using ServiceExt trait (current implementation)
    // logger_service.serve(transport).await?;

    // Option 2: Using serve_server function (Discord-style)
    let server = rmcp::serve_server(logger_service, transport).await?;
    server.waiting().await?;

    Ok(())
}
