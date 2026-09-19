use kernuxd::{
    DaemonEndpoint, DaemonServer, MAX_AUTH_BOOTSTRAP_BYTES, ProbeMetadata, ServerConfig,
    SessionAuthConfig, parse_owner_bootstrap,
};
use std::{env, error::Error, ffi::OsString, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint_arg = required_endpoint_argument()?;
    let auth = read_owner_bootstrap()?;
    let endpoint = platform_endpoint(endpoint_arg)?;
    let revision = option_env!("KERNUX_BUILD_REVISION").unwrap_or("unavailable");
    let metadata = ProbeMetadata::new(env!("CARGO_PKG_VERSION"), revision)?;
    let (server, _shutdown) =
        DaemonServer::bind(endpoint, metadata, auth, ServerConfig::default())?;
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

fn read_owner_bootstrap() -> Result<SessionAuthConfig, Box<dyn Error>> {
    let mut bytes = Vec::with_capacity(MAX_AUTH_BOOTSTRAP_BYTES);
    std::io::stdin()
        .lock()
        .take((MAX_AUTH_BOOTSTRAP_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;

    if bytes.is_empty()
        || bytes.len() > MAX_AUTH_BOOTSTRAP_BYTES
        || bytes.last() != Some(&b'\n')
        || bytes[..bytes.len() - 1].contains(&b'\n')
    {
        return Err("invalid owner session bootstrap".into());
    }
    bytes.pop();
    let value = std::str::from_utf8(&bytes).map_err(|_| "invalid owner session bootstrap")?;
    Ok(parse_owner_bootstrap(value)?)
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
