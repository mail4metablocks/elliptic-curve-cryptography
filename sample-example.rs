use rand::{thread_rng, Rng};
use secp256k1::{Message, PublicKey, SecretKey, Signature};
use secp256k1::Secp256k1;

fn main() {
    // Create a new context for interacting with the secp256k1 curve.
    let secp = Secp256k1::new();

    // Generate a new random secret key.
    let mut rng = thread_rng();
    let secret_key = SecretKey::new(&secp, &mut rng);

    // Use the secret key to create a public key.
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Hash the message that we want to sign.
    let message = Message::from_slice(&[1, 2, 3]).unwrap();

    // Sign the message using the secret key.
    let signature = secp.sign(&message, &secret_key);

    // Verify the signature using the public key.
    let result = secp.verify(&message, &signature, &public_key);
    println!("Signature is valid: {}", result.is_ok());
}
