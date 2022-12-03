use std::{os::fd::AsRawFd, str};

use color_eyre::eyre;
use nix::{
    sys::socket::{connect, UnixAddr},
    unistd::{close, read, write},
};
use socket2::{Domain, Socket, Type};

#[derive(clap::Subcommand)]
pub enum Commands {
    Monitors,
    Workspaces,
    Clients,
    ActiveWindow,
    Layers,
    Devices,
    Dispatch { dispatcher: String, arg: String },
    Keyword { keyword: String, arg: String },
    Version,
    Kill,
    Splash,
    HyprPaper { command: String, arg: String },
    Reload,
    SetCursor { theme: String, size: String },
    GetOption,
    CursorPos,
    Output { mode: String, name: String },
}

impl Commands {
    pub fn request(&self, json: bool) -> eyre::Result<()> {
        match self {
            Commands::Dispatch { dispatcher, arg } => {
                dispatch_request(&with_json(dispatcher, json), arg)
            }
            Commands::Keyword { keyword, arg } => keyword_request(&with_json(keyword, json), arg),
            Commands::HyprPaper { command, arg } => {
                hyprpaper_request(&with_json(command, json), arg)
            }
            Commands::SetCursor { theme, size } => {
                set_cursor_request(&with_json(theme, json), size)
            }
            Commands::Output { mode, name } => output_request(mode, name),
            _ => request(&self.request_str(json)),
        }
    }

    fn request_str(&self, json: bool) -> String {
        with_json(
            match self {
                Commands::Monitors => "/monitors",
                Commands::Workspaces => "/workspaces",
                Commands::Clients => "/clients",
                Commands::ActiveWindow => "/activewindow",
                Commands::Layers => "/layers",
                Commands::Devices => "/devices",
                Commands::Version => "/version",
                Commands::Kill => "/kill",
                Commands::Splash => "/splash",
                Commands::Reload => "/reload",
                Commands::GetOption => "/getoption",
                Commands::CursorPos => "/cursorpos",
                _ => "",
            },
            json,
        )
    }
}

fn with_json(str: &str, json: bool) -> String {
    match json {
        true => format!("j {str}"),
        false => str.to_string(),
    }
}

fn write_arg_to_socket(arg: &str) -> eyre::Result<(usize, Socket)> {
    let server_socket = Socket::new(Domain::UNIX, Type::STREAM, None)?;

    // Get the instance signature
    let instance_sig_str = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")?;

    let socket_path = format!("/tmp/hypr/{instance_sig_str}/.socket.sock");
    let server_address = UnixAddr::new(socket_path.as_bytes())?;

    connect(server_socket.as_raw_fd(), &server_address)?;

    let size_written = write(server_socket.as_raw_fd(), arg.as_bytes())?;

    Ok((size_written, server_socket))
}

fn request(arg: &str) -> eyre::Result<()> {
    let (_, server_socket) = write_arg_to_socket(arg)?;

    let mut buffer = [0; 8192];
    let mut size_read = read(server_socket.as_raw_fd(), &mut buffer)?;

    let mut reply = str::from_utf8(&buffer)?.to_string();

    while size_read == 8192 {
        size_read = read(server_socket.as_raw_fd(), &mut buffer)?;
        reply += str::from_utf8(&buffer)?;
    }

    close(server_socket.as_raw_fd())?;

    log::info!("{}", reply);

    Ok(())
}

fn request_hyprpaper(arg: &str) -> eyre::Result<()> {
    let (_, socket) = write_arg_to_socket(arg)?;

    let mut buffer = [0; 8192];
    let _ = read(socket.as_raw_fd(), &mut buffer)?;

    close(socket.as_raw_fd())?;

    log::info!("{}", str::from_utf8(&buffer)?);

    Ok(())
}

fn dispatch_request(dispatcher: &str, arg: &str) -> eyre::Result<()> {
    let rq = format!("/dispatch {dispatcher} {arg}");

    request(&rq)
}

fn keyword_request(keyword: &str, arg: &str) -> eyre::Result<()> {
    let rq = format!("/keyword {keyword} {arg}");

    request(&rq)
}

fn hyprpaper_request(command: &str, arg: &str) -> eyre::Result<()> {
    let rq = format!("{command} {arg}");

    request_hyprpaper(&rq)
}

fn set_cursor_request(theme: &str, size: &str) -> eyre::Result<()> {
    let rq = format!("set_cursor {theme} {size}");

    request(&rq)
}

fn output_request(mode: &str, name: &str) -> eyre::Result<()> {
    let rq = format!("output {mode} {name}");

    request(&rq)
}

fn batch_request(arg: &str, json: bool) -> eyre::Result<()> {
    let req = with_json(
        &arg.trim()
            .split(';')
            .flat_map(|s: &str| {
                let s = s.trim();

                match s.is_empty() {
                    true => None,
                    false => Some(format!("/{}", s)),
                }
            })
            .collect::<Vec<_>>()
            .join("; "),
        json,
    );

    let rq = format!("[[BATCH]]{}", req);

    request(&rq)
}

pub fn execute(batch: Option<String>, command: Option<Commands>, json: bool) -> eyre::Result<()> {
    match (batch, command) {
        (Some(batch), None) => batch_request(&batch, json),
        (None, Some(command)) => command.request(json),
        _ => Err(eyre::eyre!(
            "Either use the batch flag or use a command, check usage with \"hyprr -h/--help\""
        )),
    }
}
