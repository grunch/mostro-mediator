# MostroMediator

**MostroMediator** is a Rust program demonstrating a cryptographic mechanism for secure peer-to-peer (P2P) communication and dispute resolution, built on the [Nostr protocol](https://nostr.com/) using the `nostr-sdk` library. It leverages Elliptic Curve Diffie-Hellman (ECDH) to generate a shared key between two parties (e.g., Alice and Bob), which can be voluntarily shared with a third party to resolve disputes in systems like [Mostro](https://mostro.network/). The program encrypts messages using this shared key, ensuring privacy, authenticity, and the ability to reveal the truth if needed.

## Features

- **Shared Key Generation**: Creates a shared secret between two parties using ECDH (secp256k1).
- **Message Encryption**: Wraps messages in a simplified, non-standard NIP-59-like "gift wrap" event, signed by the sender and encrypted to the shared key.
- **Dispute Resolution**: Allows both parties to share the shared key with a mediator to decrypt and verify messages in case of a dispute.
- **Nostr Integration**: Built with `nostr-sdk` for key management, event creation, and verification.

## Use Case

In P2P marketplaces like Mostro, disputes between buyers and sellers can occur. MostroMediator enables secure communication between parties, with the option to reveal encrypted messages to a trusted third party if a dispute arises. This ensures transparency and accountability without compromising initial privacy.

## Prerequisites

- Rust (stable recommended, e.g., `1.75.0` or later).

## Installation

Clone the repository:

```bash
    git clone https://github.com/grunch/MostroMediator.git
    cd MostroMediator
```

Run the program:

```bash
    cargo run
```

## Usage

The program currently demonstrates a hardcoded example with Alice and Bob:

- Alice and Bob generate a shared key using their private and public keys.
- Alice encrypts a message ("Let’s reestablish the peer-to-peer nature of Bitcoin!") to the shared key.
- Bob (or any party with the shared key) decrypts and verifies the message.
- In a dispute, Alice and Bob can share the shared key with a mediator to decrypt and inspect the message.

## Example output:

```
Alice PubKey: npub1qqq98sa5wucc9e7ycxmjkfedxjlqr06yzjn2yhye39mu294ydgqsf8r490
Shared PubKey: npub1yvxg2xdp66jrf58hz6mk6p7n9rfrw7nqpfvzuva8q282d2pwypdsdy7l6r
Shared private key: def6633a53d07d1e829484c4d4bdbbeed2f4b14c21743e63871c174338e39475
Bob PubKey: npub1qqqqntjul70kh2ds29v7chk43sv87kyzafmus8k4m5v3vvnj5htshl66x6
Outer event: {...}
Inner event: {...}
```

To adapt this for real-world use:

- Replace hardcoded keys with dynamic inputs.
- Integrate with a Nostr relay for event transmission.
- Extend mostro_wrap and mostro_unwrap for additional functionality.

## Code Structure

- main(): Orchestrates the key generation, encryption, and decryption process.
- mostro_wrap(): Creates a signed and encrypted "gift wrap" event.
- mostro_unwrap(): Decrypts and verifies the inner event using the shared key.

## How It Works

- Key Setup: Alice and Bob each have a keypair (private/public).
- Shared Key: Generated via ECDH using Alice’s private key and Bob’s public key (or vice versa).
- Encryption: Alice signs a message and encrypts it to the shared key using a custom NIP-59-like wrapper.
- Decryption: Bob uses the shared key to decrypt and verify the message.
- Dispute Resolution: If a dispute occurs, both parties can share the shared key with a mediator, who can decrypt and verify the message to determine the truth.

## License

This project is licensed under the MIT License. See LICENSE for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## Acknowledgments

- Built with nostr-sdk.
- Inspired by Nostr NIP-59 and Mostro.
- Thanks to the Nostr and Rust communities!
