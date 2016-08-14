use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static NTHREADS: i32 = 10;

fn main() {
    threads();
    channels();
    child_process();
    pipes();
    wait();
}

fn threads() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}

fn channels() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
    }
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }
    println!("{:?}", ids);
}

fn child_process() {
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was:\n{}", s);
    }
}

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn pipes() {
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_) => println!("sent program to wc"),
    }
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => println!("wc responded with:\n{}", s),
    }
}

fn wait() {
    let mut child = std::process::Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of main");
}
