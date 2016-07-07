extern crate clap;
extern crate regex;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("magic-packet", Shell::Bash, env!("OUT_DIR"));
}
