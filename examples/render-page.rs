//! Stub example. Future milestones fill this in to actually render the page.
//! For now it just confirms the workspace links, reads the file, and prints
//! a summary.

use std::{env, fs, process};

fn main() {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: render-page <html-file>");
        process::exit(2);
    };

    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            process::exit(1);
        }
    };

    println!("loaded {} ({} bytes)", path, bytes.len());
    println!("(rendering not yet implemented; see ROADMAP milestones A1–A8)");
}
