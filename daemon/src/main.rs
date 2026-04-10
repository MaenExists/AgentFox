mod browser;

use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::ExitCode;

use agentfox_protocol::{LOG_PATH, Request, Response, SOCKET_PATH};
use browser::Browser;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let browser = Browser::new()?;

    if fs::metadata(SOCKET_PATH).is_ok() {
        fs::remove_file(SOCKET_PATH).map_err(|err| format!("failed to remove stale socket: {err}"))?;
    }

    let listener = UnixListener::bind(SOCKET_PATH).map_err(|err| format!("failed to bind socket: {err}"))?;
    log_line("daemon started")?;

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

    let request: Request = serde_json::from_str(line.trim()).map_err(|err| format!("failed to decode request: {err}"))?;
    log_line(&format!("request: {request:?}"))?;

    let (response, should_quit) = match request {
        Request::Ping => (Response::ok_message("pong"), false),
        Request::Quit => (Response::ok_message("shutting down"), true),
        Request::Open { url } => match browser.open(&url) {
            Ok(page) => (
                Response::Ok {
                    message: None,
                    url: Some(page.url),
                    title: Some(page.title),
                    text: None,
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
                    result: None,
                    elements: Some(snapshot.elements),
                },
                false,
            ),
            Err(error) => (Response::error(error), false),
        },
        Request::Click { .. } => (Response::error("click not implemented yet"), false),
        Request::Fill { .. } => (Response::error("fill not implemented yet"), false),
        Request::Text { .. } => (Response::error("text not implemented yet"), false),
        Request::Eval { code } => match browser.eval(&code) {
            Ok(result) => (
                Response::Ok {
                    message: None,
                    url: None,
                    title: None,
                    text: None,
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
