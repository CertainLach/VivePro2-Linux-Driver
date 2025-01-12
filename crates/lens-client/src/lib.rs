use std::{
	env::{self, vars_os},
	ffi::{OsStr, OsString},
	io,
	path::{Path, PathBuf},
	process::{Command, ExitStatus, Stdio},
	result,
};

use lens_protocol::ServerClient;
use serde_json::Value;
use tracing::{info, warn};

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
	#[error("proton failed with code {0} while creating prefix")]
	CantCreateProtonPrefix(ExitStatus),
}

type Result<T> = result::Result<T, Error>;

// TODO: Pass config on startup
pub fn start_lens_server(config: Value) -> Result<ServerClient> {
	let server = find_server().ok_or(Error::CantFindServer)?;
	info!("using lens server at {server:?}");

	if let Some(wine) = env::var_os("WINE") {
		let res = start_lens_server_with(wine, false, server, config)?;
		info!("server is working");
		Ok(res)
	} else {
		'proton: {
			if let Some(steamvr_path) = env::var_os("STEAM_COMPAT_INSTALL_PATH") {
				let mut path = PathBuf::from(steamvr_path);
				// Drop /SteamVR
				path.pop();
				let mut name = env::var_os("PROTON_VERSION").map(|v| {
					let mut full = OsString::from("Proton ");
					full.push(v.as_os_str());
					full
				});
				if name.is_none() {
					let readdir = std::fs::read_dir(&path)
						.expect("parent exists, there should be no other reason to fail");
					let mut candidates = Vec::new();
					for item in readdir {
						let Ok(entry) = item else {
							continue;
						};
						// Proton is ascii name
						let file_name = entry.file_name();
						let Some(name) = file_name.to_str() else {
							continue;
						};
						if !name.starts_with("Proton ") {
							continue;
						}
						candidates.push((name.to_owned(), file_name))
					}
					// Try to find latest version by semverish comparison
					candidates.sort_unstable_by(|a, b| a.0.cmp(&b.0));
					if let Some(version) = candidates.into_iter().last() {
						name = Some(version.1)
					}
				}
				if let Some(name) = name {
					path.push(name);
					path.push("proton");
					info!("trying {} as proton", path.display());

					if let Some(steamvr_compat_path) = env::var_os("STEAM_COMPAT_DATA_PATH") {
						let mut proton_prefix_path = PathBuf::from(steamvr_compat_path);
						proton_prefix_path.push("pfx");

						ensure_proton_prefix_created(&path, &proton_prefix_path)?
					} else {
						warn!("missing proton prefix environment variable");
					}

					let res = start_lens_server_with(path, true, &server, config.clone());
					match res {
						Err(Error::Io(io)) if io.kind() == io::ErrorKind::NotFound => {
							// Only possible if wine not exists
							break 'proton;
						}
						_ => {}
					}
					let res = res?;
					return Ok(res);
				} else {
					warn!("failed to find proton")
				}
				// .unwrap_or_else(|| {
				// });
			}
		}
		for wine in ["wine64", "wine"] {
			info!("trying {wine} as wine");
			let res = start_lens_server_with(wine, false, &server, config.clone());
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
	is_proton: bool,
	server_path: impl AsRef<Path>,
	config: Value,
) -> Result<ServerClient> {
	let server_path = server_path.as_ref();
	if !server_path.exists() {
		return Err(Error::ServerExeNotFound(server_path.to_owned()));
	}

	let mut child = Command::new(wine);
	if is_proton {
		child
			.arg("runinprefix")
			.arg("start.exe")
			.arg("/wait")
			.arg("/i")
			.arg("/b")
			.arg("/unix");
	}
	child
		// fixme slows down responses
		.env("WINEDEBUG", "fixme-all")
		.arg(server_path)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::inherit());
	let child = child.spawn()?;

	Ok(ServerClient::open(child, config)?)
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

fn ensure_proton_prefix_created(proton_path: &Path, prefix_path: &Path) -> Result<()> {
	if !prefix_path.exists() {
		info!("initializing proton prefix directory");

		if let Ok(status) = Command::new(proton_path).arg("run").spawn()?.wait() {
			if status.success() {
				info!("prefix created successfully")
			} else {
				return Err(Error::CantCreateProtonPrefix(status));
			}
		};
	}

	Ok(())
}
