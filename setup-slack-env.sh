#!/bin/bash

# Slack Conversation Logger Setup Script
# This script sets up the Slack bot configuration for Claude Code MCP

CONFIG_DIR="$HOME/.claude"
CONFIG_FILE="$CONFIG_DIR/slack-config.json"
EXAMPLE_FILE="slack-config.example.json"

# Create .claude directory if it doesn't exist
if [ ! -d "$CONFIG_DIR" ]; then
    echo "Creating $CONFIG_DIR directory..."
    mkdir -p "$CONFIG_DIR"
fi

# Check if example file exists
if [ ! -f "$EXAMPLE_FILE" ]; then
    echo "Error: $EXAMPLE_FILE not found in current directory"
    exit 1
fi

# Check if config already exists
if [ -f "$CONFIG_FILE" ]; then
    echo "Warning: $CONFIG_FILE already exists."
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Setup cancelled."
        exit 0
    fi
fi

# Copy the example config
cp "$EXAMPLE_FILE" "$CONFIG_FILE"
echo "Created $CONFIG_FILE"

# Instructions for the user
echo
echo "Setup complete! Next steps:"
echo "1. Edit $CONFIG_FILE with your Slack bot credentials:"
echo "   - bot_token: Your Slack bot token (starts with xoxb-)"
echo "   - channel_id: The channel ID to log conversations (starts with C)"
echo "   - thread_name: Thread name for conversation logs (optional)"
echo
echo "2. Run 'source ./load-slack-env.sh' to load the configuration"
echo
echo "3. Add to Claude Code with:"
echo "   claude mcp add -s user slack-conversation-logger slack-conversation-logger \\"
echo "     -e SLACK_TOKEN=\"\$SLACK_TOKEN\" \\"
echo "     -- --channel-id \"\$LOG_CHANNEL_ID\" --thread-name \"\$LOG_THREAD_NAME\""
echo
echo "For more information on getting Slack bot credentials, see:"
echo "https://api.slack.com/apps"