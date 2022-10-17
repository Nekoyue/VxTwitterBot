use teloxide::types::{MessageEntity, MessageEntityKind};
use teloxide::{
    dispatching::update_listeners::webhooks,
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
    Bot,
};
use url::{ParseError, Url};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting VxTwitter Bot...");

    // set bot token here
    // for unix shell: `export TELOXIDE_TOKEN=<Your token here>`
    let bot = Bot::from_env();

    // Binding address for webhook, using 127.0.0.1:12221 for my setup
    let webhook_addr = ([127, 0, 0, 1], 12221).into();

    // Url of webhook endpoint from reverse proxy (e.g. nginx/ngrok)
    // HTTPS with a valid certificate is required
    // reverse proxy need to run on port 443, 80, 88, or 8443
    let webhook_url = "https://example.com:443/vxtwitterbot/".parse().unwrap();

    let webhook_listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(webhook_addr, webhook_url),
    )
    .await
    .expect("Couldn't setup webhook");

    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |bot: Bot, q: InlineQuery| async move {
            let query_url = parse_url(&q.query);

            if query_url.is_err() {
                return respond(());
            }
            let query_url: Url = query_url.unwrap();

            // vxtwitter.com
            let vxtwitter = InlineQueryResultArticle::new(
                "00".to_string(),
                "Click to send",
                InputMessageContent::Text(
                    // result message starts with a `zero-width space`
                    InputMessageContentText::new(format!(
                        "​https://twitter.com{}",
                        query_url.path()
                    ))
                    // hyperlink the `zero-width space` for link preview
                    .entities(vec![MessageEntity::new(
                        MessageEntityKind::TextLink {
                            url: Url::parse(&format!("https://vxtwitter.com{}", query_url.path()))
                                .unwrap(),
                        },
                        0,
                        1,
                    )]),
                ),
            )
            .description(format!("https://vxtwitter.com{}", query_url.path()));

            // c.vxtwitter.com (to combine multiple images)
            let cvxtwitter = InlineQueryResultArticle::new(
                "01".to_string(),
                "Combine multiple images (if any)",
                InputMessageContent::Text(
                    InputMessageContentText::new(format!(
                        "​https://twitter.com{}",
                        query_url.path()
                    ))
                    .entities(vec![MessageEntity::new(
                        MessageEntityKind::TextLink {
                            url: Url::parse(&format!(
                                "https://c.vxtwitter.com{}",
                                query_url.path()
                            ))
                            .unwrap(),
                        },
                        0,
                        1,
                    )]),
                ),
            )
            .description(format!("https://c.vxtwitter.com{}", query_url.path()));

            let results = vec![
                InlineQueryResult::Article(cvxtwitter),
                InlineQueryResult::Article(vxtwitter),
            ];

            let response = bot.answer_inline_query(&q.id, results).send().await;

            if let Err(err) = response {
                log::error!("Error in handler: {:?}", err);
            }

            respond(())
        },
    ));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            webhook_listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}

/// Parse and validate a Url from a string
///
/// # Errors
///
/// [`ParseError`] will be returned for any invalid strings not meeting the conditions (see code below)
fn parse_url(url: &str) -> Result<Url, ParseError> {
    // Url is valid
    let parsed_url = Url::parse(url)?;

    // Domain is `twitter.com`
    match parsed_url.domain() {
        Some(host) => {
            if host != "twitter.com" {
                return Err(ParseError::InvalidDomainCharacter);
            }
        }
        None => return Err(ParseError::EmptyHost),
    }

    // Has a non-empty path
    if parsed_url.path() == "/" {
        return Err(ParseError::RelativeUrlWithoutBase);
    }

    Ok(parsed_url)
}
