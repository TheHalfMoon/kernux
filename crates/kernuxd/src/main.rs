use kernuxd::{DaemonEndpoint, DaemonServer, ProbeMetadata, ServerConfig};
use std::{env, error::Error, ffi::OsString};

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint_arg = required_endpoint_argument()?;
    let endpoint = platform_endpoint(endpoint_arg)?;
    let revision = option_env!("KERNUX_BUILD_REVISION").unwrap_or("unavailable");
    let metadata = ProbeMetadata::new(env!("CARGO_PKG_VERSION"), revision)?;
    let (server, _shutdown) = DaemonServer::bind(endpoint, metadata, ServerConfig::default())?;
    server.serve()?;
    Ok(())
}

fn required_endpoint_argument() -> Result<OsString, Box<dyn Error>> {
    let mut arguments = env::args_os().skip(1);
    let endpoint = arguments
        .next()
        .ok_or("kernuxd requires exactly one local endpoint argument")?;
    if arguments.next().is_some() {
        return Err("kernuxd requires exactly one local endpoint argument".into());
    }
    Ok(endpoint)
}

#[cfg(unix)]
fn platform_endpoint(argument: OsString) -> Result<DaemonEndpoint, Box<dyn Error>> {
    Ok(DaemonEndpoint::unix_runtime_dir(argument)?)
}

#[cfg(windows)]
fn platform_endpoint(argument: OsString) -> Result<DaemonEndpoint, Box<dyn Error>> {
    let pipe_name = argument
        .into_string()
        .map_err(|_| "Windows pipe name must be valid Unicode")?;
    Ok(DaemonEndpoint::windows_pipe(pipe_name)?)
}
