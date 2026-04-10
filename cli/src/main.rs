use std::env;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process::ExitCode;

use agentfox_protocol::{Request, Response, SOCKET_PATH};

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
    let request = parse_request(env::args().skip(1).collect())?;
    let mut stream =
        UnixStream::connect(SOCKET_PATH).map_err(|err| format!("failed to connect to daemon: {err}"))?;
    let payload = serde_json::to_string(&request).map_err(|err| format!("failed to encode request: {err}"))?;
    stream
        .write_all(payload.as_bytes())
        .and_then(|_| stream.write_all(b"\n"))
        .map_err(|err| format!("failed to send request: {err}"))?;

    let mut response = String::new();
    let mut reader = BufReader::new(stream);
    reader
        .read_line(&mut response)
        .map_err(|err| format!("failed to read response: {err}"))?;

    let response: Response =
        serde_json::from_str(response.trim()).map_err(|err| format!("failed to decode response: {err}"))?;
    let pretty = serde_json::to_string_pretty(&response).map_err(|err| format!("failed to render response: {err}"))?;
    println!("{pretty}");
    Ok(())
}

fn parse_request(args: Vec<String>) -> Result<Request, String> {
    match args.as_slice() {
        [command] if command == "ping" => Ok(Request::Ping),
        [command] if command == "snap" => Ok(Request::Snap),
        [command] if command == "quit" => Ok(Request::Quit),
        [command, url] if command == "open" => Ok(Request::Open { url: url.clone() }),
        [command, element_id] if command == "click" => Ok(Request::Click {
            element_id: element_id.clone(),
        }),
        [command, element_id] if command == "text" => Ok(Request::Text {
            element_id: element_id.clone(),
        }),
        [command, element_id, text] if command == "fill" => Ok(Request::Fill {
            element_id: element_id.clone(),
            text: text.clone(),
        }),
        [command, code] if command == "eval" => Ok(Request::Eval { code: code.clone() }),
        _ => Err(
            "usage: afox <open|snap|click|fill|text|eval|ping|quit> [args]".to_string(),
        ),
    }
}
