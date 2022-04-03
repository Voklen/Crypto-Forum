use ed25519_dalek::*;

pub fn hash(bytes: &[u8]) -> [u8;64] {
	let mut h: Sha512 = Sha512::new();
	let mut hash: [u8; 64] = [0u8; 64];

	h.update(bytes);
	hash.copy_from_slice(h.finalize().as_slice());
	hash
}
