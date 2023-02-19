use std::{os::fd::AsRawFd, str};

use color_eyre::eyre;
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use nix::{
    sys::socket::{connect, UnixAddr},
    unistd::{close, read, write},
};
use socket2::{Domain, Socket, Type};

pub(crate) const USAGE: &str = r"
usage hyprr [(opt)flags] [command] [(opt) args] # same as hyprctl
      hyppr -g (--gui) # gui application

commands:
    monitors
    workspaces
    clients
    activewindow
    layers
    devices
    binds
    dispatch
    keyword
    version
    kill
    splash
    hyprpaper
    reload
    setcursor
    getoption
    cursorpos
    switchxkblayout
    seterror
    setprop

flags:
    -g, --gui -> run the gui application
    -j -> output in JSON
    --batch -> execute a batch of commands, separated by ';'
";

pub fn execute(args: &[String]) -> eyre::Result<()> {
    let (full_request, full_args, _, err) = args
        .iter()
        .fold_while(
            (String::new(), String::new(), true, false),
            |(mut req, mut args, mut parse_args, mut err), arg| {
                if arg.starts_with("--") {
                    parse_args = false;

                    return Continue((req, args, parse_args, err));
                }

                if parse_args && arg.starts_with('-') && !is_number(arg, true) {
                    if arg == "-j" && !args.contains('j') {
                        args += "j";
                    } else if arg == "--batch" {
                        req = "--batch ".to_string();
                    } else {
                        err = true;
                        return Done((req, args, parse_args, err));
                    }
                }

                req.push_str(arg);
                req.push(' ');

                Continue((req, args, parse_args, err))
            },
        )
        .into_inner();

    if err || full_request.is_empty() {
        return Err(eyre::eyre!("{USAGE}"));
    }

    let full_request = format!("{full_args}/{}", full_request.trim_end());

    kiam::when! {
        full_request.contains("/--batch") => batch_request(&full_request),
        [
            "/monitors",
            "/clients",
            "/workspaces",
            "/activewindow",
            "/layers",
            "/version",
            "/kill",
            "/splash",
            "/devices",
            "/reload",
            "/getoption",
            "/binds",
            "/cursorpos",
            "/animations",
        ].iter().any(|s| full_request.contains(s)) => request(&full_request, 0),
        full_request.contains("/switchxkblayout") => request(&full_request, 2),
        full_request.contains("/seterror") => request(&full_request, 1),
        full_request.contains("/setprop") => request(&full_request, 3),
        full_request.contains("/output") => output_request(args),
        full_request.contains("/setcursor") => set_cursor_request(args),
        full_request.contains("/dispatch") => dispatch_request(args),
        full_request.contains("/keyword") => keyword_request(args),
        full_request.contains("/hyprpaper") => hyprpaper_request(args),
        full_request.contains("/--help") => { println!("{USAGE}"); Ok(()) },
        _ => Err(eyre::eyre!("{USAGE}")),
    }
}

fn is_number(str: &str, allow_float: bool) -> bool {
    if str.is_empty() {
        return false;
    }

    str.chars()
        .all(|c| c.is_ascii_digit() || c == '-' || (allow_float && c == '.'))
}

fn batch_request(arg: &str) -> eyre::Result<()> {
    let rq = format!(
        "[[BATCH]]{}",
        &arg[arg.chars().position(|c| c == ' ').unwrap() + 1..]
    );

    request(&rq, 0)
}

fn request(arg: &str, min_args: usize) -> eyre::Result<()> {
    let args = arg.chars().filter(|c| *c == ' ').count();

    if args < min_args {
        return Err(eyre::eyre!(
            "Not enough arguments, expected at least {min_args}"
        ));
    }

    let Ok(server_socket) = Socket::new(Domain::UNIX, Type::STREAM, None) else {
        return Err(eyre::eyre!("Couldn't open a socket (1)"));
    };

    let Ok(instance_sig) = std::env::var("HYPRLAND_INSTANCE_SIGNATURE") else {
        return Err(eyre::eyre!(
            "HYPRLAND_INSTANCE_SIGNATURE was not set! (Is Hyprland running?)"
        ));
    };

    let socket_path = format!("/tmp/hypr/{instance_sig}/.socket.sock");
    let server_address = UnixAddr::new(socket_path.as_str())?;

    let Ok(_) = connect(server_socket.as_raw_fd(), &server_address) else {
        return Err(eyre::eyre!("Couldn't connect to {socket_path}. (3)"));
    };

    let Ok(_) = write(server_socket.as_raw_fd(), arg.as_bytes()) else {
        return Err(eyre::eyre!("Couldn't write (4)"));
    };

    let mut buffer = [0; 8192];
    let Ok(mut size_written) = read(server_socket.as_raw_fd(), &mut buffer) else {
        return Err(eyre::eyre!("Couldn't read (5)"));
    };

    let mut reply = String::from_utf8_lossy(&buffer[..size_written]).to_string();

    while size_written == 8192 {
        match read(server_socket.as_raw_fd(), &mut buffer) {
            Ok(sw) => size_written = sw,
            Err(_) => {
                return Err(eyre::eyre!("Couldn't read (5)"));
            }
        }

        reply.push_str(&String::from_utf8_lossy(&buffer[..size_written]));
    }

    close(server_socket.as_raw_fd())?;

    println!("{reply}");

    Ok(())
}

fn output_request(args: &[String]) -> eyre::Result<()> {
    if args.len() < 4 {
        return Err(eyre::eyre!(
            r"
            Usage: hyprr output <mode> <name>
                   creates / destroys a fake output
                   with create, name is the backend name to use (available: auto, x11, wayland, headless)
                   with destroy, name is the output name to destroy
            "
        ));
    }

    let rq = format!("output {} {}", args[2], args[3]);

    request(&rq, 0)
}

fn set_cursor_request(args: &[String]) -> eyre::Result<()> {
    if args.len() < 4 {
        return Err(eyre::eyre!(
            r"
            Usage: hyprr setcursor <theme> <size>
                   Sets the sursor theme for everything except GTK and reloads the cursor
            "
        ));
    }

    let rq = format!("setcursor {} {}", args[2], args[3]);

    request(&rq, 0)
}

fn dispatch_request(args: &[String]) -> eyre::Result<()> {
    if args.len() < 3 {
        return Err(eyre::eyre!(
            r"
            Usage: hyprr dispatch <dispatcher> <arg>
                   Execute a hyprland keybind dispatcher with the given argument
            "
        ));
    }

    let rq = args[2..].iter().fold("/dispatch".to_string(), |rq, arg| {
        if !arg.starts_with("--") {
            return rq;
        }

        format!("{rq} {arg}")
    });

    request(&rq, 0)
}

fn keyword_request(args: &[String]) -> eyre::Result<()> {
    if args.len() < 4 {
        return Err(eyre::eyre!(
            r"
            Usage: hyprr keyword <keyword> <arg>
                   Execute a hyprland keyword with the given argument
            "
        ));
    }

    let rq = args[2..]
        .iter()
        .fold("/keyword".to_string(), |rq, arg| format!("{rq} {arg}"));

    request(&rq, 0)
}

fn hyprpaper_request(args: &[String]) -> eyre::Result<()> {
    if args.len() < 4 {
        return Err(eyre::eyre!(
            r"
            Usage: hyprr hyprpaper <command> <arg>
                   Execute a hyprpaper command with the given argument
            "
        ));
    }

    let rq = format!("{} {}", args[2], args[3]);

    request_hyprpaper(&rq)
}

fn request_hyprpaper(arg: &str) -> eyre::Result<()> {
    let Ok(server_socket) = Socket::new(Domain::UNIX, Type::STREAM, None) else {
        return Err(eyre::eyre!("Couldn't open a socket (1)"));
    };

    // Get instance signature
    let Ok(instance_sig) = std::env::var("HYPRLAND_INSTANCE_SIGNATURE") else {
        return Err(eyre::eyre!("HYPRLAND_INSTANCE_SIGNATURE was not set! (Is Hyprland running?)"));
    };

    let socket_path = format!("/tmp/hypr/{instance_sig}/.hyprpaper.sock");
    let server_address = UnixAddr::new(socket_path.as_str())?;

    let Ok(_) = connect(server_socket.as_raw_fd(), &server_address) else {
        return Err(eyre::eyre!("Couldn't connect to {socket_path}. (3)"));
    };

    let Ok(_) = write(server_socket.as_raw_fd(), arg.as_bytes()) else {
        return Err(eyre::eyre!("Couldn't write (4)"));
    };

    let mut buffer = [0; 8192];
    let Ok(_) = read(server_socket.as_raw_fd(), &mut buffer) else {
        return Err(eyre::eyre!("Couldn't read (5)"));
    };

    close(server_socket.as_raw_fd())?;

    println!("{}", String::from_utf8_lossy(&buffer));

    Ok(())
}
