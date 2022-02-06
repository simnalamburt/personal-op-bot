use anyhow::{anyhow, Result};
use argon2::verify_encoded;
use clap::Parser;

use futures::prelude::*;
use tokio::task::spawn_blocking;
use tracing::{debug, info, trace};

use irc::client::prelude::*;
use irc::proto::Command;

const HASH: &str = "$argon2id$v=19$m=65536,t=3,p=4$eW9sb3N3YWd5b2xvc3dhZw$kUIWwfRAKBVZRuQUOovTLISQ2R9MdEhmQhGXBqG9iiQ";

/// A simple and ignorant IRC OP maintenance bot
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Path of the config file
    #[clap(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let mut client = Client::new(args.config).await?;
    let mut stream = client.stream()?;

    client.identify()?;
    info!("{} is ready", client.current_nickname());

    while let Some(msg) = stream.try_next().await? {
        debug!("{}", msg.to_string().trim_end());
        trace!("{:#?}", msg);

        // Reference: https://modern.ircdocs.horse/#client-messages
        // TODO: handle '?'s properly
        match msg.command {
            Command::JOIN(chanlist, ..) => {
                client.send_privmsg(&chanlist, "옵을 잃어버렸습니다. 옵을 주세요! CC @김지현")?;
                info!("Joinned {}", chanlist);
            }
            Command::ChannelMODE(chan, modes) => {
                // React on /op and /deop
                let current = client.current_nickname();
                for mode in modes {
                    match mode {
                        Mode::Plus(ChannelMode::Oper, Some(nick)) if nick == current => {
                            client.send_privmsg(&chan, "감사합니다!")?;
                            info!("Opped in {}", chan);
                        }
                        Mode::Minus(ChannelMode::Oper, Some(nick)) if nick == current => {
                            client.send_privmsg(&chan, "옵을 빼앗겼습니다. CC @김지현")?;
                            info!("Deopped in {}", chan);
                        }
                        _ => {}
                    }
                }
            }
            Command::PRIVMSG(msgtarget, text) if msgtarget == client.current_nickname() => {
                // React on OP request
                if let Some(Prefix::Nickname(sender, ..)) = msg.prefix {
                    // Verify password
                    let task = spawn_blocking(move || verify_encoded(HASH, text.as_bytes()));
                    if !task.await?? {
                        continue;
                    }

                    // OP the sender on correct password
                    let channels = client.list_channels().ok_or_else(|| {
                        anyhow!("'nochanlists' feature was given to the 'irc' dependency")
                    })?;
                    let modes = &[Mode::Plus(ChannelMode::Oper, Some(sender.clone()))];
                    for chan in &channels {
                        client.send_mode(chan, modes)?;
                    }
                    client.send_privmsg(&sender, "인증 성공, 옵 부여됨.")?;
                    info!("Opped {} in {} as requested", sender, channels.join(", "));
                }
            }
            Command::KICK(chan, nick, comment) if nick == client.current_nickname() => {
                // Automatically rejoin channel if the bot was kicked
                client.send_join(&chan)?;
                let reason = comment.unwrap_or_else(|| "No reason given".to_string());
                info!("Kicked from {} (reason: {}), rejoining ...", chan, reason);
            }
            _ => {}
        }
    }

    info!("Finished");
    Ok(())
}
