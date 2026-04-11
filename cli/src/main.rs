use std::env;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process::ExitCode;

use agentfox_protocol::{Request, Response, SOCKET_PATH};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "--help" || args[0] == "-h" || args[0] == "help" {
        println!("{}", usage());
        return Ok(());
    }

    if args[0] == "--version" || args[0] == "-v" {
        println!("afox version {}", VERSION);
        return Ok(());
    }

    let request = parse_request(args)?;
    
    let stream = match connect_with_retry(3) {
        Ok(s) => s,
        Err(_) => {
            spawn_daemon()?;
            connect_with_retry(20).map_err(|err| {
                format!("afoxd failed to start or is taking too long: {err}")
            })?
        }
    };

    let mut stream = stream;
    let payload = serde_json::to_string(&request).map_err(|err| format!("failed to encode request: {err}"))?;
    
    stream
        .write_all(payload.as_bytes())
        .and_then(|_| stream.write_all(b"\n"))
        .map_err(|err| format!("failed to send request: {err}"))?;

    let mut response_line = String::new();
    let mut reader = BufReader::new(stream);
    reader
        .read_line(&mut response_line)
        .map_err(|err| format!("failed to read response: {err}"))?;

    if response_line.trim().is_empty() {
        return Err("daemon closed connection without response".to_string());
    }

    let response: Response =
        serde_json::from_str(response_line.trim()).map_err(|err| format!("failed to decode response: {err}"))?;
    
    match response {
        Response::Ok { markdown: Some(md), .. } => {
            println!("{md}");
        }
        Response::Ok { text: Some(txt), .. } => {
            println!("{txt}");
        }
        Response::Ok { url: Some(url), title: Some(title), .. } => {
            println!("## {title}\nURL: {url}");
        }
        Response::Ok { message: Some(msg), .. } => {
            println!("{msg}");
        }
        Response::Ok { result: Some(res), .. } => {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Response::Ok { .. } => {
            let pretty = serde_json::to_string_pretty(&response).map_err(|err| format!("failed to render response: {err}"))?;
            println!("{pretty}");
        }
        Response::Error { error } => {
            return Err(error);
        }
    }
    
    Ok(())
}

fn parse_request(args: Vec<String>) -> Result<Request, String> {
    let cmd = args[0].as_str();
    match cmd {
        "search" if args.len() == 2 => Ok(Request::Search { query: args[1].clone() }),
        "open" if args.len() == 2 => Ok(Request::Open { url: args[1].clone() }),
        "click" if args.len() == 2 => Ok(Request::Click { element_id: args[1].clone() }),
        "text" if args.len() == 2 => Ok(Request::Text { element_id: args[1].clone() }),
        "fill" if args.len() == 3 => Ok(Request::Fill { element_id: args[1].clone(), text: args[2].clone() }),
        "eval" if args.len() == 2 => Ok(Request::Eval { code: args[1].clone() }),
        "snap" => Ok(Request::Snap),
        "view" => Ok(Request::View),
        "ping" => Ok(Request::Ping),
        "quit" => Ok(Request::Quit),
        _ => Err(format!("invalid command or arguments for '{}'. See 'afox help'.", cmd)),
    }
}

fn connect_with_retry(retries: usize) -> Result<UnixStream, String> {
    for i in 0..retries {
        match UnixStream::connect(SOCKET_PATH) {
            Ok(stream) => return Ok(stream),
            Err(_) => {
                if i < retries - 1 {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }
    Err(format!("could not connect to {}", SOCKET_PATH))
}

fn spawn_daemon() -> Result<(), String> {
    use std::process::{Command, Stdio};

    let current_exe = env::current_exe().ok();
    let afoxd_path = current_exe
        .as_ref()
        .and_then(|p| p.parent())
        .map(|p| p.join("afoxd"))
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "afoxd".to_string());

    Command::new(afoxd_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| {
            format!(
                "failed to auto-start afoxd daemon. error: {}",
                err
            )
        })?;

    Ok(())
}

fn usage() -> String {
    format!(
        "AgentFox CLI (afox) v{}\n\
        High-velocity browser interaction for AI agents.\n\n\
        USAGE:\n\
          afox <COMMAND> [ARGS]\n\n\
        COMMANDS:\n\
          search <query>      Smart navigation: URL or search query\n\
          open <url>          Navigate to a specific URL\n\
          snap                Get a semantic JSON snapshot of the page\n\
          view                Get a semantic Markdown snapshot of the page\n\
          click <id>          Perform a realistic click on an element\n\
          fill <id> <text>    Input text into a field\n\
          text <id>           Extract clean text from an element\n\
          eval <code>         Execute arbitrary JavaScript\n\
          ping                Check if the daemon is alive\n\
          quit                Shutdown the background daemon\n\n\
        FLAGS:\n\
          -h, --help          Show this help message\n\
          -v, --version       Show version information\n\n\
        The daemon (afoxd) is automatically started if not already running.",
        VERSION
    )
}
