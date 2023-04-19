use crate::{base64::*, custom_types::*, read};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;

pub fn write_messages(link: &str, data: Vec<Message>) -> Result<String, Error> {
	let write_data = get_write_data(link, data)?;
	let data_as_toml = toml::to_string(&write_data).map_err(Error::toml_serialization)?;
	upload_to_ipns(link, data_as_toml.clone())?;
	Ok(data_as_toml)
}

fn get_write_data(file: &str, data: Vec<Message>) -> Result<FullFile, Error> {
	let mut new_messages = message_to_file_message(data);

	// Read existing messages (see Decisions.md for explanation)
	let existing_file = read::parse_full_file(file)?;
	let mut messages = existing_file.messages;
	messages.append(&mut new_messages);

	Ok(FullFile {
		header: existing_file.header,
		messages,
	})
}

pub fn message_to_file_message(data: Vec<Message>) -> Vec<FileMessage> {
	data.into_iter()
		.map(|f| {
			let prev_hash = bytes_to_hex(&f.prev_hash);
			let signature = bytes_to_hex(&f.signature.to_bytes());
			let public_key = bytes_to_hex(&f.public_key.to_bytes());
			let message = f.message;
			FileMessage {
				prev_hash,
				public_key,
				message,
				signature,
			}
		})
		.collect()
}

fn upload_to_ipns(link: &str, contents: String) -> Result<(), Error> {
	let client = IpfsClient::default();
	let data = Cursor::new(contents);
	let executor = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.map_err(Error::StdIo)?;

	let result_future = client.add(data);
	let result = executor.block_on(result_future).map_err(Error::IPFS)?;
	let name = format!("/ipfs/{}", result.name);

	//TODO Make saving repo specific
	let repo_name = None;
	let publish_future = client.name_publish(&name, false, None, None, repo_name);
	executor.block_on(publish_future).map_err(Error::IPFS)?;
	Ok(())
}

pub fn new_ipns(name: &str) -> Result<String, Error> {
	let client = IpfsClient::default();
	let executor = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.map_err(Error::StdIo)?;

	let key = ipfs_api_backend_hyper::KeyType::Ed25519;
	let keygen_future = client.key_gen(name, key, 64);
	let result = executor.block_on(keygen_future).map_err(Error::IPFS)?;
	let ipns_link = result.id;

	Ok(ipns_link)
}
