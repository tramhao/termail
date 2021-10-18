#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]
// #![warn(clippy::restriction)]
/**
 * MIT License
 *
 * termail - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
mod app;
mod config;
mod ui;
mod utils;

use app::App;
use config::TermailConfig;
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut config = TermailConfig::default();
    config.load().unwrap_or_default();

    let mut args: Vec<String> = std::env::args().collect();
    // match args.len() {}

    args.remove(0);
    let mut should_exit = false;
    for i in args {
        let i = i.as_str();
        match i {
            "-v" | "--version" => {
                println!("Termusic version is: {}", VERSION);
                should_exit = true;
            }

            "-h" | "--help" => {
                println!(
                    r"Termusic help:
Usage: termusic [DIRECTORY] [OPTIONS]
-v or --version print version and exit.
-h or --help print this message and exit.
directory: start termusic with directory.
no arguments: start termusic with ~/.config/termusic/config.toml"
                );
                should_exit = true;
            }

            _ => {
                let p = Path::new(i);
                let mut p_string = String::new();
                if p.exists() {
                    if p.has_root() {
                        if let Ok(p1) = p.canonicalize() {
                            p_string = p1.as_path().to_string_lossy().to_string();
                        }
                    } else if let Ok(p_base) = std::env::current_dir() {
                        let p2 = p_base.join(&p);
                        if let Ok(p3) = p2.canonicalize() {
                            p_string = p3.as_path().to_string_lossy().to_string();
                        }
                    }
                    config.music_dir_from_cli = Some(p_string);
                } else {
                    println!(
                        r"Unknown arguments
Termusic help:
Usage: termusic [DIRECTORY] [OPTIONS]
-v or --version print version and exit.
-h or --help print this message and exit.
directory: start termusic with directory.
no arguments: start termusic with ~/.config/termusic/config.toml"
                    );
                    should_exit = true;
                }
            }
        }
    }

    if should_exit {
        return;
    }

    // glib::set_application_name("termusic");
    // glib::set_prgname(Some("termusic"));
    let mut app: App = App::new(config);
    app.run();
}
