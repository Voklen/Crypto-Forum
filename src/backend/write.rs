use crate::{custom_types::*, read};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;

/// Append the messages to the GitArk repo at the link
pub fn write_messages(link: &str, messages: Vec<Message>) -> Result<(), Error> {
	let name = &ipns_link_to_key(link);
	let write_data = get_write_data(link, messages)?;
	let data_as_toml = toml::to_string(&write_data).map_err(Error::toml_serialization)?;
	upload_to_ipns(name, data_as_toml)?;
	Ok(())
}

fn get_write_data(link: &str, messages: Vec<Message>) -> Result<FullFile, Error> {
	// Read existing messages (see Decisions.md for explanation)
	let existing_file = read::get_repo(link)?;

	let mut new_messages = messages.into_iter().map(message_to_file_message).collect();
	let mut messages = existing_file.messages;
	messages.append(&mut new_messages);

	Ok(FullFile {
		header: existing_file.header,
		messages,
	})
}

pub fn message_to_file_message(m: Message) -> FileMessage {
	let prev_hash = m.prev_hash_string();
	let signature = m.signature_string();
	let public_key = m.public_key_string();
	let body = m.body;
	FileMessage {
		prev_hash,
		public_key,
		body,
		signature,
	}
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

/// Creates a new IPNS link and returns the link to it
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
