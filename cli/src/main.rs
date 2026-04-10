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
    
    let stream = UnixStream::connect(SOCKET_PATH).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            "afoxd daemon is not running. Start it with 'afoxd &'".to_string()
        } else {
            format!("failed to connect to daemon: {err}")
        }
    })?;

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
    
    let pretty = serde_json::to_string_pretty(&response).map_err(|err| format!("failed to render response: {err}"))?;
    println!("{pretty}");
    
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
        "ping" => Ok(Request::Ping),
        "quit" => Ok(Request::Quit),
        _ => Err(format!("invalid command or arguments for '{}'. See 'afox help'.", cmd)),
    }
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
          click <id>          Perform a realistic click on an element\n\
          fill <id> <text>    Input text into a field\n\
          text <id>           Extract clean text from an element\n\
          eval <code>         Execute arbitrary JavaScript\n\
          ping                Check if the daemon is alive\n\
          quit                Shutdown the background daemon\n\n\
        FLAGS:\n\
          -h, --help          Show this help message\n\
          -v, --version       Show version information\n\n\
        The daemon (afoxd) must be running for these commands to work.",
        VERSION
    )
}
