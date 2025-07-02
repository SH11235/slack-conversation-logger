# Slack Conversation Logger

An MCP (Model Context Protocol) server that logs conversations to Slack threads. This is a Rust implementation that allows AI assistants to log important interactions, decisions, or context to a designated Slack channel.

![スクリーンショット 2025-07-02 231306](https://github.com/user-attachments/assets/182e6515-0440-4e78-833b-2ec691113c1e)

## Features

- Logs messages to Slack threads with role-based formatting
- Supports human, assistant, and system message types with color coding
- Automatically creates a new thread for each working directory
- Thread-safe message posting
- Optional context metadata for each message
- Configuration via environment variables or command-line arguments

## Prerequisites

- Rust (latest stable version)
- A Slack workspace with a bot app configured
- Required Slack bot permissions:
  - `chat:write`
  - `chat:write.public` (if posting to public channels)
  - `channels:read` (to verify channel access)

## Installation

### Using Cargo (Recommended)

```bash
cargo install --git https://github.com/SH11235/slack-conversation-logger.git
```

### Building from source

1. Clone the repository:
```bash
git clone https://github.com/SH11235/slack-conversation-logger.git
cd slack-conversation-logger
```

2. Install locally:
```bash
cargo install --path .
```

Or build without installing:
```bash
cargo build --release
# Binary will be at ./target/release/slack-conversation-logger
```

## Configuration

### Quick Setup for Claude Code

1. If you want to use the setup scripts, clone the repository:
   ```bash
   git clone https://github.com/SH11235/slack-conversation-logger.git
   cd slack-conversation-logger
   ./setup-slack-env.sh
   ```

2. Edit `~/.claude/slack-config.json` with your Slack credentials:
   ```json
   {
     "bot_token": "xoxb-your-slack-bot-token",
     "channel_id": "C1234567890",
     "thread_name": "Conversation Log"  // Optional, defaults to "Conversation Log"
   }
   ```

3. Load environment variables (if using the config file):
   ```bash
   source ./load-slack-env.sh
   ```

4. Add the MCP server to Claude Code:
   ```bash
   # For global use (available in all projects):
   claude mcp add -s user slack-conversation-logger slack-conversation-logger \
     -e SLACK_TOKEN="$SLACK_TOKEN" \
     -- --channel-id "$LOG_CHANNEL_ID" --thread-name "$LOG_THREAD_NAME"
   
   # Or manually specify values:
   claude mcp add -s user slack-conversation-logger slack-conversation-logger \
     -e SLACK_TOKEN="xoxb-your-actual-slack-bot-token" \
     -- --channel-id "C1234567890" --thread-name "Conversation Log"
   
   # For local project use (default):
   claude mcp add slack-conversation-logger slack-conversation-logger \
     -e SLACK_TOKEN="$SLACK_TOKEN" \
     -- --channel-id "$LOG_CHANNEL_ID" --thread-name "$LOG_THREAD_NAME"
   ```

### Manual Configuration

The server requires the following configuration via environment variables or command-line arguments:

- `SLACK_TOKEN`: Your Slack bot token (required)
- `LOG_CHANNEL_ID`: The channel ID to log to (required)
- `LOG_THREAD_NAME`: Thread name prefix (optional, default: "Conversation Log")

## Setting up Slack Bot

1. Go to https://api.slack.com/apps and create a new app
2. Choose "From scratch" and give your app a name
3. Under "OAuth & Permissions", add the required bot token scopes:
   - `chat:write`
   - `chat:write.public`
   - `channels:read`
4. Install the app to your workspace
5. Copy the Bot User OAuth Token (starts with `xoxb-`)
6. Invite the bot to your target channel: `/invite @your-bot-name`
7. Get the channel ID (right-click channel → View channel details → ID at bottom)

## Usage

### Running the server

With environment variables:
```bash
export SLACK_TOKEN="xoxb-your-token"
export LOG_CHANNEL_ID="C1234567890"
export LOG_THREAD_NAME="My Conversation Log"  # Optional
cargo run
```

Or with command-line arguments:
```bash
cargo run -- --slack-token "xoxb-your-token" --channel-id "C1234567890"
```

Use the built binary:
```bash
./target/release/slack-conversation-logger --slack-token "xoxb-your-token" --channel-id "C1234567890"
```

### MCP Integration

Add to your Claude Desktop configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "slack-logger": {
      "command": "/path/to/slack-conversation-logger",
      "env": {
        "SLACK_TOKEN": "xoxb-your-token",
        "LOG_CHANNEL_ID": "C1234567890",
        "LOG_THREAD_NAME": "Claude Conversations"
      }
    }
  }
}
```

### Available Tools

The server provides one tool:

#### `log_conversation`

Logs a message to the configured Slack channel.

Parameters:
- `role` (required): The message sender role - "human", "assistant", or "system"
- `message` (required): The message content to log
- `context` (optional): Additional context or metadata

Example usage in Claude:
```
I'll log this decision to Slack for future reference.

<use_tool>
<tool_name>log_conversation</tool_name>
<parameters>
{
  "role": "assistant",
  "message": "Implemented Slack conversation logger based on Discord version",
  "context": "Created new Rust MCP server with slack-morphism client"
}
</parameters>
</use_tool>
```

## Message Formatting

Messages are posted as Slack attachments with:
- Color coding: Blue (human), Green (assistant), Gray (system)
- Author name showing the role
- Timestamp in UTC
- Optional context field
- All messages in a thread named after the current working directory

## Troubleshooting

### "Invalid auth" error
- Verify your bot token starts with `xoxb-`
- Ensure the token has not been revoked
- Check that all required scopes are added

### "channel_not_found" error
- Verify the channel ID is correct (should start with C)
- Ensure the bot has been invited to the channel
- For private channels, the bot needs to be a member

### "missing_scope" error
- Add the required OAuth scopes in your Slack app settings
- Reinstall the app to your workspace after adding scopes

## Development

To modify the logger:

1. Edit the source files in `src/`
2. Key files:
   - `main.rs`: Entry point and configuration
   - `slack.rs`: Slack API client implementation  
   - `logger.rs`: MCP tool implementation

3. Test locally:
```bash
cargo test
cargo run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## CLAUDE.md Configuration

To encourage proper logging practices when using this MCP server with Claude, add the following content to your `CLAUDE.md` file:

```markdown
# Slack Conversation Logger Rules

## Important Message Logging Rules

Please use slack-conversation-logger MCP to log messages in the following cases:

### 1. User Messages (human)
- Task start/change/completion instructions
- Important decisions or confirmations  
- Error reports or issue identification
- Feature requests and requirements

### 2. Assistant Messages (assistant)
- Task completion reports
- Important suggestions or solutions
- Error resolution methods
- Summary of significant changes made
- Key implementation decisions

### 3. System Messages (system)
- Critical errors or warnings
- Important environment changes
- Security-related notifications
- Build or test failures

## Logging Format

```
mcp__slack-conversation-logger__log_conversation(
  message: "Actual message content",
  role: "human" | "assistant" | "system",
  context: "Brief context description"
)
```

## When to Log
- Beginning of significant tasks
- Completion of major milestones
- Important architecture or design decisions
- Bug discoveries and their resolutions
- Any changes that affect the project's functionality

## Cases Not Requiring Logs
- Simple acknowledgments
- Intermediate progress reports  
- Temporary debug outputs
- Minor code formatting changes
- Routine file reads or searches
```

This configuration helps maintain an audit trail of important project decisions and progress in your Slack workspace.
