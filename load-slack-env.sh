#!/bin/bash

# Slack Conversation Logger Environment Loader
# Source this script to load Slack configuration as environment variables
# Usage: source ./load-slack-env.sh

CONFIG_FILE="$HOME/.claude/slack-config.json"

# Check if config file exists
if [ ! -f "$CONFIG_FILE" ]; then
    echo "Error: $CONFIG_FILE not found"
    echo "Please run ./setup-slack-env.sh first"
    return 1 2>/dev/null || exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed"
    echo "Please install jq: sudo apt-get install jq (Ubuntu/Debian) or brew install jq (macOS)"
    return 1 2>/dev/null || exit 1
fi

# Read configuration and export as environment variables
export SLACK_TOKEN=$(jq -r '.bot_token' "$CONFIG_FILE")
export LOG_CHANNEL_ID=$(jq -r '.channel_id' "$CONFIG_FILE")
export LOG_THREAD_NAME=$(jq -r '.thread_name // "Conversation Log"' "$CONFIG_FILE")

# Verify that required variables are set
if [ "$SLACK_TOKEN" = "null" ] || [ -z "$SLACK_TOKEN" ]; then
    echo "Error: bot_token not found in $CONFIG_FILE"
    return 1 2>/dev/null || exit 1
fi

if [ "$LOG_CHANNEL_ID" = "null" ] || [ -z "$LOG_CHANNEL_ID" ]; then
    echo "Error: channel_id not found in $CONFIG_FILE"
    return 1 2>/dev/null || exit 1
fi

echo "Slack environment variables loaded successfully:"
echo "  SLACK_TOKEN: [HIDDEN]"
echo "  LOG_CHANNEL_ID: $LOG_CHANNEL_ID"
echo "  LOG_THREAD_NAME: $LOG_THREAD_NAME"