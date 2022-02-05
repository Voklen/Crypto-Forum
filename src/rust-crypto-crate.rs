extern crate crypto;
fn main() {
    let seed: &[u8] = b"cool seed";
    let x = crypto::ed25519::keypair(seed);
    println!("Hello, world! {:?}", x);
}