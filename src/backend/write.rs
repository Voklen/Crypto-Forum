use crate::{base64::*, custom_types::*, read};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;

pub fn write_messages(link: &str, data: Vec<Message>) -> Result<String, Error> {
	let name = &ipns_link_to_key(link);
	let write_data = get_write_data(link, data)?;
	let data_as_toml = toml::to_string(&write_data).map_err(Error::toml_serialization)?;
	upload_to_ipns(name, data_as_toml.clone())?;
	Ok(data_as_toml)
}

fn get_write_data(link: &str, data: Vec<Message>) -> Result<FullFile, Error> {
	// Read existing messages (see Decisions.md for explanation)
	let existing_file = read::parse_full_file(link)?;

	let mut new_messages = message_to_file_message(data);
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

fn upload_to_ipns(key: &str, contents: String) -> Result<(), Error> {
	let client = IpfsClient::default();
	let data = Cursor::new(contents);
	let executor = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.map_err(Error::StdIo)?;

	let result_future = client.add(data);
	let result = executor.block_on(result_future).map_err(Error::IPFS)?;
	let ipfs_link = format!("/ipfs/{}", result.name);

	let publish_future = client.name_publish(&ipfs_link, false, None, None, Some(key));
	executor.block_on(publish_future).map_err(Error::IPFS)?;
	Ok(())
}

pub fn new_ipns(contents: &FullFile) -> Result<String, Error> {
	let client = IpfsClient::default();
	let executor = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.map_err(Error::StdIo)?;

	let temp_key = "ThisIsATempGitArkNameIfYouAreSeeingThisSomethingWentWrong";
	let key_type = ipfs_api_backend_hyper::KeyType::Ed25519;
	let keygen_future = client.key_gen(temp_key, key_type, 64);
	let result = executor.block_on(keygen_future).map_err(Error::IPFS)?;
	let ipns_link = result.id;

	write_to_new_ipns(temp_key, contents)?;
	rename_ipns_key(temp_key, &ipns_link)?;

	Ok(ipns_link)
}

fn write_to_new_ipns(key: &str, contents: &FullFile) -> Result<(), Error> {
	let data_as_toml = toml::to_string(contents).map_err(Error::toml_serialization)?;
	upload_to_ipns(key, data_as_toml)
}

fn rename_ipns_key(key_name: &str, link: &str) -> Result<(), Error> {
	let client = IpfsClient::default();
	let executor = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.map_err(Error::StdIo)?;

	let new_name = ipns_link_to_key(link);
	let key_rename_future = client.key_rename(key_name, &new_name, false);
	executor.block_on(key_rename_future).map_err(Error::IPFS)?;
	Ok(())
}

pub fn delete_key(key: &str) -> Result<(), Error> {
	let client = IpfsClient::default();
	let executor = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.map_err(Error::StdIo)?;

	let command = client.key_rm(key);
	executor.block_on(command).map_err(Error::IPFS)?;

	Ok(())
}
