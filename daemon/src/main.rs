mod browser;

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::ExitCode;

use agentfox_protocol::{LOG_PATH, Request, Response, SOCKET_PATH};
use browser::Browser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        if args[0] == "--help" || args[0] == "-h" || args[0] == "help" {
            println!("{}", usage());
            return ExitCode::SUCCESS;
        }
        if args[0] == "--version" || args[0] == "-v" {
            println!("afoxd version {}", VERSION);
            return ExitCode::SUCCESS;
        }
    }

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("daemon error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    println!("Starting AgentFox Daemon v{}...", VERSION);
    let browser = Browser::new()?;

    if fs::metadata(SOCKET_PATH).is_ok() {
        fs::remove_file(SOCKET_PATH).map_err(|err| format!("failed to remove stale socket: {err}"))?;
    }

    let listener = UnixListener::bind(SOCKET_PATH).map_err(|err| {
        format!("failed to bind socket {}. Is another instance running? error: {}", SOCKET_PATH, err)
    })?;
    
    log_line(&format!("daemon v{} started", VERSION))?;
    println!("Daemon listening on {}", SOCKET_PATH);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                log_line(&format!("accept error: {err}"))?;
                continue;
            }
        };

        let should_quit = handle_client(stream, &browser)?;
        if should_quit {
            break;
        }
    }

    fs::remove_file(SOCKET_PATH).ok();
    log_line("daemon stopped")?;
    println!("Daemon shutdown complete.");
    Ok(())
}

fn handle_client(mut stream: UnixStream, browser: &Browser) -> Result<bool, String> {
    let mut line = String::new();
    {
        let mut reader = BufReader::new(&stream);
        reader
            .read_line(&mut line)
            .map_err(|err| format!("failed to read request: {err}"))?;
    }

    let request: Request = match serde_json::from_str(line.trim()) {
        Ok(req) => req,
        Err(err) => {
            let resp = Response::error(format!("failed to decode request: {err}"));
            let payload = serde_json::to_string(&resp).unwrap();
            let _ = stream.write_all(payload.as_bytes());
            let _ = stream.write_all(b"\n");
            return Ok(false);
        }
    };
    
    log_line(&format!("request: {request:?}"))?;

    let (response, should_quit) = match request {
        Request::Ping => (Response::ok_message("pong"), false),
        Request::Quit => (Response::ok_message("shutting down"), true),
        Request::Search { query } => match browser.search(&query) {
            Ok(page) => (
                Response::Ok {
                    message: None,
                    url: Some(page.url),
                    title: Some(page.title),
                    text: None,
                    markdown: None,
                    result: None,
                    elements: None,
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
        Request::Open { url } => match browser.open(&url) {
            Ok(page) => (
                Response::Ok {
                    message: None,
                    url: Some(page.url),
                    title: Some(page.title),
                    text: None,
                    markdown: None,
                    result: None,
                    elements: None,
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
        Request::Snap => match browser.snap() {
            Ok(snapshot) => (
                Response::Ok {
                    message: None,
                    url: Some(snapshot.url),
                    title: Some(snapshot.title),
                    text: None,
                    markdown: None,
                    result: None,
                    elements: Some(snapshot.elements),
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
        Request::View => match browser.view() {
            Ok(markdown) => (
                Response::Ok {
                    message: None,
                    url: None,
                    title: None,
                    text: None,
                    markdown: Some(markdown),
                    result: None,
                    elements: None,
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
        Request::Click { element_id } => match browser.click(&element_id) {
            Ok(page) => (
                Response::Ok {
                    message: None,
                    url: Some(page.url),
                    title: Some(page.title),
                    text: None,
                    markdown: None,
                    result: None,
                    elements: None,
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
        Request::Fill { element_id, text } => match browser.fill(&element_id, &text) {
            Ok(()) => (Response::ok_message("filled element"), false),
            Err(error) => (Response::error(error), false),
        },
        Request::Text { element_id } => match browser.text(&element_id) {
            Ok(text) => (
                Response::Ok {
                    message: None,
                    url: None,
                    title: None,
                    text: Some(text),
                    markdown: None,
                    result: None,
                    elements: None,
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
        Request::Eval { code } => match browser.eval(&code) {
            Ok(result) => (
                Response::Ok {
                    message: None,
                    url: None,
                    title: None,
                    text: None,
                    markdown: None,
                    result: Some(result),
                    elements: None,
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
    };

    let payload = serde_json::to_string(&response).map_err(|err| format!("failed to encode response: {err}"))?;
    stream
        .write_all(payload.as_bytes())
        .and_then(|_| stream.write_all(b"\n"))
        .map_err(|err| format!("failed to write response: {err}"))?;
    Ok(should_quit)
}

fn log_line(message: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
        .map_err(|err| format!("failed to open log file: {err}"))?;
    writeln!(file, "{message}").map_err(|err| format!("failed to write log entry: {err}"))?;
    Ok(())
}

fn usage() -> String {
    format!(
        "AgentFox Daemon (afoxd) v{}\n\
        The persistent browser engine for AI agents.\n\n\
        USAGE:\n\
          afoxd [FLAGS]\n\n\
        FLAGS:\n\
          -h, --help          Show this help message\n\
          -v, --version       Show version information\n\n\
        The daemon runs in the foreground by default. Use 'afoxd &' to run in the background.",
        VERSION
    )
}
