use clap::{Args, Parser, Subcommand};
use nostr::PublicKey;
use sentrystr_collector::{EventCollector, EventFilter, PrivateMessageConfig, Result};

fn parse_tag(s: &str) -> std::result::Result<(String, String), String> {
    match s.split_once('=') {
        Some((key, value)) => Ok((key.to_string(), value.to_string())),
        None => Err(format!("Invalid tag format '{}'. Expected 'key=value'", s)),
    }
}

fn parse_level(level_str: &str) -> std::result::Result<sentrystr::Level, String> {
    match level_str.to_lowercase().as_str() {
        "debug" => Ok(sentrystr::Level::Debug),
        "info" => Ok(sentrystr::Level::Info),
        "warning" => Ok(sentrystr::Level::Warning),
        "error" => Ok(sentrystr::Level::Error),
        "fatal" => Ok(sentrystr::Level::Fatal),
        _ => Err("Invalid level".to_string()),
    }
}

fn build_private_message_config(
    send_to: Option<String>,
    send_min_level: Option<String>,
    use_nip17: bool,
) -> Result<Option<PrivateMessageConfig>> {
    if let Some(recipient_str) = send_to {
        let recipient_pubkey = PublicKey::parse(&recipient_str).map_err(|e| {
            sentrystr_collector::CollectorError::Collection(format!(
                "Invalid recipient public key: {}",
                e
            ))
        })?;

        let min_level = if let Some(level_str) = send_min_level {
            Some(parse_level(&level_str).map_err(sentrystr_collector::CollectorError::Collection)?)
        } else {
            None
        };

        Ok(Some(PrivateMessageConfig {
            recipient_pubkey,
            min_level,
            use_nip17,
        }))
    } else {
        Ok(None)
    }
}

#[derive(Parser)]
#[command(name = "sentrystr-collector")]
#[command(about = "A collector for SentryStr events from Nostr network")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Collect(CollectArgs),
    Subscribe(SubscribeArgs),
}

#[derive(Args)]
struct CollectArgs {
    #[arg(short, long, help = "Public key to collect events from")]
    author: Option<String>,

    #[arg(
        short,
        long,
        default_value = "100",
        help = "Maximum number of events to collect"
    )]
    limit: usize,

    #[arg(short, long, help = "Relay URLs", default_values = &["wss://relay.damus.io"])]
    relays: Vec<String>,

    #[arg(
        long,
        help = "Filter by log level (debug, info, warning, error, fatal)"
    )]
    level: Option<String>,

    #[arg(long, help = "Filter by service tag")]
    service: Option<String>,

    #[arg(long, help = "Filter by environment tag")]
    environment: Option<String>,

    #[arg(long, help = "Filter by component tag")]
    component: Option<String>,

    #[arg(long, help = "Filter by severity tag")]
    severity: Option<String>,

    #[arg(long, help = "Filter by custom Nostr tag (format: key=value)", value_parser = parse_tag)]
    tag: Vec<(String, String)>,

    #[arg(long, help = "Send events as private messages to this public key")]
    send_to: Option<String>,

    #[arg(
        long,
        help = "Minimum level to send as private message (debug, info, warning, error, fatal)"
    )]
    send_min_level: Option<String>,

    #[arg(long, help = "Use NIP-17 for private messages (default: NIP-44)")]
    use_nip17: bool,
}

#[derive(Args)]
struct SubscribeArgs {
    #[arg(short, long, help = "Public key to subscribe to events from")]
    author: Option<String>,

    #[arg(short, long, help = "Relay URLs", default_values = &["wss://relay.damus.io"])]
    relays: Vec<String>,

    #[arg(
        long,
        help = "Filter by log level (debug, info, warning, error, fatal)"
    )]
    level: Option<String>,

    #[arg(long, help = "Filter by service tag")]
    service: Option<String>,

    #[arg(long, help = "Filter by environment tag")]
    environment: Option<String>,

    #[arg(long, help = "Filter by component tag")]
    component: Option<String>,

    #[arg(long, help = "Filter by severity tag")]
    severity: Option<String>,

    #[arg(long, help = "Filter by custom Nostr tag (format: key=value)", value_parser = parse_tag)]
    tag: Vec<(String, String)>,

    #[arg(long, help = "Send events as private messages to this public key")]
    send_to: Option<String>,

    #[arg(
        long,
        help = "Minimum level to send as private message (debug, info, warning, error, fatal)"
    )]
    send_min_level: Option<String>,

    #[arg(long, help = "Use NIP-17 for private messages (default: NIP-44)")]
    use_nip17: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Collect(args) => {
            let mut collector = EventCollector::new(args.relays).await?;

            if let Some(pm_config) =
                build_private_message_config(args.send_to, args.send_min_level, args.use_nip17)?
            {
                collector = collector.with_private_messaging(pm_config)?;
            }

            let mut filter = EventFilter::new().with_limit(args.limit);

            if let Some(author_str) = args.author {
                let author = PublicKey::parse(&author_str).map_err(|e| {
                    sentrystr_collector::CollectorError::Collection(format!(
                        "Invalid public key: {}",
                        e
                    ))
                })?;
                filter = filter.with_author(author);
            }

            if let Some(level_str) = args.level {
                let level = parse_level(&level_str)
                    .map_err(sentrystr_collector::CollectorError::Collection)?;
                filter = filter.with_level(level);
            }

            if let Some(service) = args.service {
                filter = filter.with_service_filter(service);
            }

            if let Some(environment) = args.environment {
                filter = filter.with_environment_filter(environment);
            }

            if let Some(component) = args.component {
                filter = filter.with_component_filter(component);
            }

            if let Some(severity) = args.severity {
                filter = filter.with_severity_filter(severity);
            }

            for (key, value) in args.tag {
                filter = filter.with_nostr_tag(key, value);
            }

            println!("Collecting events...");
            let events = collector.collect_events(filter).await?;

            println!("Found {} events:", events.len());
            for event in events {
                println!("---");
                println!("Event ID: {}", event.nostr_event_id);
                println!("Author: {}", event.author);
                println!("Timestamp: {}", event.event.timestamp);
                println!("Level: {:?}", event.event.level);
                println!("Message: {:?}", event.event.message);
                println!("Tags: {:?}", event.event.tags);
            }

            collector.disconnect().await?;
        }
        Commands::Subscribe(args) => {
            let mut collector = EventCollector::new(args.relays).await?;

            if let Some(pm_config) =
                build_private_message_config(args.send_to, args.send_min_level, args.use_nip17)?
            {
                collector = collector.with_private_messaging(pm_config)?;
            }

            let mut filter = EventFilter::new();

            if let Some(author_str) = args.author {
                let author = PublicKey::parse(&author_str).map_err(|e| {
                    sentrystr_collector::CollectorError::Collection(format!(
                        "Invalid public key: {}",
                        e
                    ))
                })?;
                filter = filter.with_author(author);
            }

            if let Some(level_str) = args.level {
                let level = parse_level(&level_str)
                    .map_err(sentrystr_collector::CollectorError::Collection)?;
                filter = filter.with_level(level);
            }

            if let Some(service) = args.service {
                filter = filter.with_service_filter(service);
            }

            if let Some(environment) = args.environment {
                filter = filter.with_environment_filter(environment);
            }

            if let Some(component) = args.component {
                filter = filter.with_component_filter(component);
            }

            if let Some(severity) = args.severity {
                filter = filter.with_severity_filter(severity);
            }

            for (key, value) in args.tag {
                filter = filter.with_nostr_tag(key, value);
            }

            println!("Subscribing to events... (Press Ctrl+C to stop)");
            let mut rx = collector.subscribe_to_events(filter).await?;

            while let Some(event) = rx.recv().await {
                println!("---");
                println!("New Event ID: {}", event.nostr_event_id);
                println!("Author: {}", event.author);
                println!("Timestamp: {}", event.event.timestamp);
                println!("Level: {:?}", event.event.level);
                println!("Message: {:?}", event.event.message);
                println!("Tags: {:?}", event.event.tags);
                println!("Received at: {}", event.received_at);
            }

            collector.disconnect().await?;
        }
    }

    Ok(())
}
