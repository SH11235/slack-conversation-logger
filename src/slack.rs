use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;
use slack_morphism::{
    SlackApiToken, SlackApiTokenValue, SlackChannelId, SlackMessageContent,
    SlackTs,
    api::SlackApiChatPostMessageRequest,
    blocks::*,
    prelude::SlackClientHyperConnector,
};

// Type alias for cleaner code and easier maintenance
type DefaultSlackClient = slack_morphism::SlackClient<
    SlackClientHyperConnector<
        hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>
    >
>;

static THREAD_TS: OnceCell<SlackTs> = OnceCell::new();

pub struct SlackClient {
    client: DefaultSlackClient,
    token: SlackApiToken,
    channel_id: SlackChannelId,
    thread_name: String,
}

impl SlackClient {
    pub async fn new(token: String, channel_id: String, thread_name: String) -> Result<Self> {
        let client = slack_morphism::SlackClient::new(SlackClientHyperConnector::new()?);
        let token_value = SlackApiTokenValue::new(token);
        let api_token = SlackApiToken::new(token_value);
        
        Ok(Self {
            client,
            token: api_token,
            channel_id: SlackChannelId(channel_id),
            thread_name,
        })
    }
    
    pub async fn log_message(
        &self,
        role: &str,
        message: &str,
        context: Option<&str>,
    ) -> Result<()> {
        let thread_ts = match THREAD_TS.get() {
            Some(ts) => ts.clone(),
            None => {
                let ts = self.create_thread().await?;
                THREAD_TS.set(ts.clone()).map_err(|_| {
                    anyhow::anyhow!("Failed to set thread timestamp")
                })?;
                ts
            }
        };
        
        let current_time: DateTime<Utc> = Utc::now();
        let timestamp = current_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();
        
        // Create blocks for the message
        let mut blocks: Vec<SlackBlock> = vec![];
        
        // Add header with role and timestamp
        let header_text = format!("{} - {}", role.to_uppercase(), timestamp);
        blocks.push(SlackBlock::Section(
            SlackSectionBlock::new()
                .with_text(SlackBlockText::MarkDown(
                    SlackBlockMarkDownText::new(format!("*{}*", header_text))
                ))
        ));
        
        // Add message content
        blocks.push(SlackBlock::Section(
            SlackSectionBlock::new()
                .with_text(SlackBlockText::MarkDown(
                    SlackBlockMarkDownText::new(message.to_string())
                ))
        ));
        
        // Add context if provided
        if let Some(ctx) = context {
            blocks.push(SlackBlock::Context(
                SlackContextBlock::new(vec![
                    SlackContextBlockElement::Plain(
                        SlackBlockPlainText::new(format!("Context: {}", ctx))
                    )
                ])
            ));
        }
        
        // Add colored bar using a divider
        blocks.push(SlackBlock::Divider(SlackDividerBlock::new()));
        
        let content = SlackMessageContent::new().with_blocks(blocks);
        let post_req = SlackApiChatPostMessageRequest::new(
            self.channel_id.clone(),
            content
        )
        .with_thread_ts(thread_ts);
        
        let session = self.client.open_session(&self.token);
        session
            .chat_post_message(&post_req)
            .await?;
        
        Ok(())
    }
    
    async fn create_thread(&self) -> Result<SlackTs> {
        let cwd = std::env::current_dir()?
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let thread_title = format!("{} - {}", self.thread_name, cwd);
        
        let post_req = SlackApiChatPostMessageRequest::new(
            self.channel_id.clone(),
            SlackMessageContent::new()
                .with_text(format!(":thread: *{}*\n_Starting new conversation log_", thread_title))
        );
        
        let session = self.client.open_session(&self.token);
        let response = session
            .chat_post_message(&post_req)
            .await?;
        
        Ok(response.ts)
    }
}

#[async_trait::async_trait]
impl crate::logger::Logger for SlackClient {
    async fn log(&self, role: &str, message: &str, context: Option<&str>) -> Result<()> {
        self.log_message(role, message, context).await
    }
}