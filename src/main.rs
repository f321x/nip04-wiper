use nostr_sdk::prelude::*;
use tokio::time::timeout;
use std::{collections::HashSet, env, time::Duration};

// this was a 30 min tool hack, don't expect much, but it works

#[tokio::main]
async fn main() {
    // enable logging to stdout
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // get nsec from cli
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 || args[1].is_empty() {
        eprintln!("Pass your nsec as argument. ./nip04-wiper <nsec>");
        return;
    }

    let nsec = args[1].parse::<String>().unwrap();
    let keys = Keys::parse(nsec).expect("Invalid nsec");
    let client = Client::new(&keys);
    add_relays(&client).await;

    let dm_filter = Filter::new()
			.author(keys.public_key())
			.kind(Kind::EncryptedDirectMessage);
    let auto_close = SubscribeAutoCloseOptions::default().timeout(Some(Duration::from_secs(30)));
	let _subscription_id = client.subscribe(vec![dm_filter], Some(auto_close)).await.unwrap();
	let mut notifications = client.notifications();
    let mut nip04dms: HashSet<EventId> = HashSet::new();

    println!("Fetching events, this can take some time.");
    loop {
        let notification = timeout(Duration::from_secs(30), notifications.recv()).await;
        match notification {
            Ok(Ok(notification)) => {
                if let RelayPoolNotification::Event { event, .. } = notification {
                    println!("Found nip04 event: {} ", event.id);
                    nip04dms.insert(event.id);
                }
            }
            Ok(Err(_)) => break,
            Err(_) => {
                println!("Timeout reached, stopping the loop.");
                break;
            }
        }
    }
    client.unsubscribe_all().await;
    println!("Found {} nip04 dms", nip04dms.len());

    let delete_event = EventBuilder::delete_with_reason(nip04dms, "github.com/f321x/nip04-wiper".to_string()).to_event(&keys).unwrap();
    client.send_event(delete_event.clone()).await.unwrap();
    println!("Sent delete event {:#?}, done.", delete_event.id());
}

async fn add_relays(client: &Client) {
    client
        .add_relay("wss://nostr.bitcoiner.social")
        .await
        .unwrap();
    client.add_relay("wss://nostr.mom").await.unwrap();
    client.add_relay("wss://nos.lol").await.unwrap();
    client.add_relay("wss://relay.damus.io").await.unwrap();
    client.add_relay("wss://labour.fiatjaf.com/").await.unwrap();
    client.add_relay("wss://nostr.lu.ke").await.unwrap();
    client.add_relay("wss://relay.nostr.band/").await.unwrap();
    client.add_relay("wss://nostr.wine").await.unwrap();
    client.add_relay("wss://filter.nostr.wine").await.unwrap();
    client.add_relay("wss://nostr.oxtr.dev").await.unwrap();
    client.add_relay("wss://relay.mostr.pub").await.unwrap();
    client.add_relay("wss://ftp.halifax.rwth-aachen.de/nostr").await.unwrap();
    client.add_relay("wss://relay.westernbtc.com").await.unwrap();
    client.add_relay("wss://inbox.nostr.wine").await.unwrap();
    client.add_relay("wss://relay.primal.net").await.unwrap();
    client.add_write_relay("wss://sendit.nosflare.com").await.unwrap();  // blaster relay
    println!("Connecting to relays, please wait...");
    client.connect().await;
    tokio::time::sleep(Duration::from_secs(10)).await;
}
