use nostr::util::generate_shared_key;
use nostr_sdk::prelude::*;

// Alice
// Hex public key:         000053c3b4773182e7c4c1b72b272d34be01bf4414a6a25c998977c516a46a01
// Hex private key:        548f68890c49fa42f104c60352395e60ff030b0b407e955f1eed1400d6c0347a
// Npub public key:        npub1qqq98sa5wucc9e7ycxmjkfedxjlqr06yzjn2yhye39mu294ydgqsf8r490
// Nsec private key:       nsec12j8k3zgvf8ay9ugyccp4yw27vrlsxzctgplf2hc7a52qp4kqx3aq0ttwy2

// Bob
// Hex public key:         000009ae5cff9f6ba9b05159ec5ed58c187f5882ea77c81ed5dd19163272a5d7
// Hex private key:        f258e73f07386d37133718b6127f873dd7c391b8f43b331ff8254034a13d2943
// Npub public key:        npub1qqqqntjul70kh2ds29v7chk43sv87kyzafmus8k4m5v3vvnj5htshl66x6
// Nsec private key:       nsec17fvww0c88pknwyehrzmpylu88htu8ydc7sanx8lcy4qrfgfa99psdvrw0q

// Hex Shared PubKey:      27199d5878869ec3b4ae1ad5c2fed88840218a119f9ce892828b950fc96b4829
// Hex Shared private key: def6633a53d07d1e829484c4d4bdbbeed2f4b14c21743e63871c174338e39475

#[tokio::main]
async fn main() -> Result<()> {
    // Alice
    let alice_keys =
        Keys::parse("548f68890c49fa42f104c60352395e60ff030b0b407e955f1eed1400d6c0347a")?;
    // Bob
    let bob_keys = Keys::parse("f258e73f07386d37133718b6127f873dd7c391b8f43b331ff8254034a13d2943")?;
    // Show Alice bech32 public key
    let alice_pubkey = alice_keys.public_key();
    let alice_secret = alice_keys.secret_key();
    println!("Alice PubKey: {}", alice_pubkey);

    // Generate shared key for Alice
    let shared_key = generate_shared_key(alice_secret, &bob_keys.public_key())?;
    let shared_secret_key = SecretKey::from_slice(&shared_key)?;
    let shared_keys = Keys::new(shared_secret_key);
    println!("Shared PubKey: {}", shared_keys.public_key());
    println!(
        "Shared private key: {}",
        shared_keys.secret_key().to_secret_hex()
    );
    // Generate shared key for Bob
    let bob_shared_key = generate_shared_key(bob_keys.secret_key(), &alice_keys.public_key())?;
    // Check if both shared keys are the same, shared keys are not the same it panic
    assert_eq!(shared_key, bob_shared_key);
    // Show Bob bech32 public key
    let bob_pubkey = bob_keys.public_key();
    // let bob_secret = bob_keys.secret_key();
    println!("Bob PubKey: {}", bob_pubkey);

    let message = "Let’s reestablish the peer-to-peer nature of Bitcoin!";
    // We encrypt the event to the shared key and only can be decrypted by the shared key
    // and sign the inside event with the sender key, in this case Alice
    // We do this to ensure that the message is from Alice and only Bob can read it
    // But both parties can `shared` the shared key to anyone to decrypt the message
    // This is useful for p2p like Mostro where in case of a dispute the message can be decrypted
    // by a third party to know if someone is lying
    let wrapped_event = mostro_wrap(&alice_keys, shared_keys.public_key(), message, vec![]).await?;
    println!("Outer event: {:#?}", wrapped_event);

    // We decrypt the event with the shared key
    let unwrapped_event = mostro_unwrap(&shared_keys, wrapped_event).await.unwrap();
    println!("Inner event: {:#?}", unwrapped_event);

    Ok(())
}

/// Wraps a message in a non standard and simplified NIP-59 event.
/// The inner event is signed with the sender's key and encrypted to the receiver's
/// public key using an ephemeral key.
///
/// # Arguments
/// - `sender`: The sender's keys for signing the inner event.
/// - `receiver`: The receiver's public key for encryption.
/// - `message`: The message to wrap.
/// - `extra_tags`: Additional tags to include in the wrapper event.
///
/// # Returns
/// A signed `Event` representing the NON STANDARD gift wrap.
pub async fn mostro_wrap(
    sender: &Keys,
    receiver: PublicKey,
    message: &str,
    extra_tags: Vec<Tag>,
) -> Result<Event, Box<dyn std::error::Error>> {
    let inner_event = EventBuilder::text_note(message)
        .build(sender.public_key())
        .sign(sender)
        .await?;
    let keys: Keys = Keys::generate();
    let encrypted_content: String = nip44::encrypt(
        keys.secret_key(),
        &receiver,
        inner_event.as_json(),
        nip44::Version::V2,
    )
    .unwrap();

    // Build tags for the wrapper event
    let mut tags = vec![Tag::public_key(receiver)];
    tags.extend(extra_tags);

    // Create and sign the gift wrap event
    let wrapped_event = EventBuilder::new(Kind::GiftWrap, encrypted_content)
        .tags(tags)
        .custom_created_at(Timestamp::tweaked(nip59::RANGE_RANDOM_TIMESTAMP_TWEAK))
        .sign_with_keys(&keys)?;

    Ok(wrapped_event)
}

/// Unwraps an non standard NIP-59 event and retrieves the inner event.
/// The receiver uses their private key to decrypt the content.
///
/// # Arguments
/// - `receiver`: The receiver's keys for decryption.
/// - `event`: The wrapped event to unwrap.
///
/// # Returns
/// The decrypted inner `Event`.
pub async fn mostro_unwrap(
    receiver: &Keys,
    event: Event,
) -> Result<Event, Box<dyn std::error::Error>> {
    let decrypted_content = nip44::decrypt(receiver.secret_key(), &event.pubkey, &event.content)?;
    let inner_event = Event::from_json(&decrypted_content)?;

    // Verify the event before returning
    inner_event.verify()?;

    Ok(inner_event)
}
