use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

#[test]
fn worker_handles_multiple_requests_in_one_process() {
    let bin = env!("CARGO_BIN_EXE_eng");
    let mut child = Command::new(bin)
        .arg("worker")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn eng worker");

    let mut stdin = child.stdin.take().expect("worker stdin");
    let stdout = child.stdout.take().expect("worker stdout");
    let mut reader = BufReader::new(stdout);

    let req1 = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "w1",
        "op": "constant.get",
        "args": { "key": "g0" }
    });
    writeln!(stdin, "{req1}").expect("write req1");
    stdin.flush().expect("flush req1");

    let mut line = String::new();
    reader.read_line(&mut line).expect("read resp1");
    let resp1: Value = serde_json::from_str(line.trim()).expect("parse resp1");
    assert_eq!(resp1["ok"], true);
    assert_eq!(resp1["op"], "constant.get");
    assert_eq!(resp1["request_id"], "w1");

    line.clear();
    let req2 = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "w2",
        "op": "constant.get",
        "args": { "key": "pi" }
    });
    writeln!(stdin, "{req2}").expect("write req2");
    stdin.flush().expect("flush req2");
    reader.read_line(&mut line).expect("read resp2");
    let resp2: Value = serde_json::from_str(line.trim()).expect("parse resp2");
    assert_eq!(resp2["ok"], true);
    assert_eq!(resp2["op"], "constant.get");
    assert_eq!(resp2["request_id"], "w2");
    assert_ne!(resp1["value"], resp2["value"]);

    drop(stdin);
    let _ = child.wait();
}

#[test]
fn worker_returns_structured_error() {
    let bin = env!("CARGO_BIN_EXE_eng");
    let mut child = Command::new(bin)
        .arg("worker")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn eng worker");

    let mut stdin = child.stdin.take().expect("worker stdin");
    let stdout = child.stdout.take().expect("worker stdout");
    let mut reader = BufReader::new(stdout);

    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "w-err",
        "op": "fluid.prop",
        "args": { "fluid": "H2O" }
    });
    writeln!(stdin, "{req}").expect("write req");
    stdin.flush().expect("flush req");

    let mut line = String::new();
    reader.read_line(&mut line).expect("read resp");
    let resp: Value = serde_json::from_str(line.trim()).expect("parse resp");
    assert_eq!(resp["ok"], false);
    assert_eq!(resp["request_id"], "w-err");
    assert_eq!(resp["error"]["code"], "missing_arg");

    drop(stdin);
    let _ = child.wait();
}
