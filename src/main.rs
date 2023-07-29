use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

#[tokio::main]
pub async fn main() {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                twitch_irc::message::ServerMessage::Privmsg(payload) => {
                    println!("{}\n{}\n", payload.sender.name, payload.message_text);
                }
                _ => {}
            }
        }
    });

    client.join("theidofalan".to_owned()).unwrap();
    join_handle.await.unwrap();
}
