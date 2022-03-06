use std::{
	env,
	ffi::OsStr,
	io,
	path::{Path, PathBuf},
	process::{Command, Stdio},
	result,
};

use lens_protocol::Client;
use serde_json::Value;
use tracing::info;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("io error: {0}")]
	Io(#[from] io::Error),
	#[error("lens protocol error: {0}")]
	Protocol(#[from] lens_protocol::Error),
	#[error("cant find server, specify LENS_SERVER_EXE")]
	CantFindServer,
	#[error("expected server exe at {0}, but it is not found")]
	ServerExeNotFound(PathBuf),
	#[error("can't find wine in PATH")]
	WineNotFound,
}

type Result<T> = result::Result<T, Error>;

// TODO: Pass config on startup
pub fn start_lens_server(config: Value) -> Result<Client> {
	let server = find_server().ok_or(Error::CantFindServer)?;
	info!("using lens server at {server:?}");

	if let Some(wine) = env::var_os("WINE") {
		let res = start_lens_server_with(wine, server, config)?;
		info!("server is working");
		Ok(res)
	} else {
		for wine in ["wine64", "wine"] {
			info!("trying {wine} as wine");
			let res = start_lens_server_with(wine, &server, config.clone());
			match res {
				Err(Error::Io(io)) if io.kind() == io::ErrorKind::NotFound => {
					// Only possible if wine not exists
					continue;
				}
				_ => {}
			}
			let res = res?;
			return Ok(res);
		}
		Err(Error::WineNotFound)
	}
}

pub fn start_lens_server_with(
	wine: impl AsRef<OsStr>,
	server_path: impl AsRef<Path>,
	config: Value,
) -> Result<Client> {
	let server_path = server_path.as_ref();
	if !server_path.exists() {
		return Err(Error::ServerExeNotFound(server_path.to_owned()));
	}

	let child = Command::new(wine)
		// fixme slows down responses
		.env("WINEDEBUG", "fixme-all")
		.arg(server_path)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::inherit())
		.spawn()?;

	Ok(Client::open(child, config)?)
}

pub fn find_server() -> Option<PathBuf> {
	if let Some(path) = env::var_os("LENS_SERVER_EXE") {
		return Some(PathBuf::from(path));
	}
	let mut path = process_path::get_dylib_path()?;
	path.pop();
	path.push("lens-server");
	path.push("lens-server.exe");
	Some(path)
}
