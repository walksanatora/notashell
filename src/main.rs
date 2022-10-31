#![allow(clippy::let_unit_value)]

use std::process::{Command, Stdio};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional key to escape the sandbox
    #[arg(short,long)]
    key: Option<String>
}

fn load_docker() {
    {
        println!("Getting Username");
        let name = Command::new("whoami").output().expect("Unnable to get Username, Kicking!").stdout.into_boxed_slice();
        let uname = String::from_utf8_lossy(&name);
        
        println!("checking if you have a container");
        let mut has_container = false;
        let containers = Command::new("docker")
            .args(vec!["container","ls","-a"])
            .output().expect("unnable to list containers, permission issue?").stdout;
        for line in String::from_utf8_lossy(&containers.into_boxed_slice()).lines() {
            if line.contains(uname.trim()) {
                has_container = true;
                break
            }
        }
        let mut docker = if has_container {
            println!("container found, checking if running");
            let mut is_active = false;
            let containers = Command::new("docker")
                .args(vec!["container","ps"])
                .output().expect("unnable to list containers, permission issue?").stdout;
            for line in String::from_utf8_lossy(&containers.into_boxed_slice()).lines() {
                if line.contains(uname.trim()) {
                    is_active = true;
                    break
                }
            }
            if is_active {
                println!("container running, attaching");
                Command::new("docker")
                    .args(vec!["attach",uname.trim()])
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn().expect("Docker Failed to Start, Kicking!")
            } else {
                println!("container not running, starting");
                Command::new("docker")
                    .args(vec!["start","-ai",uname.trim()])
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn().expect("Docker Failed to Start, Kicking!")
            }
        } else {
            println!("No container found, lets make one");
            Command::new("docker")
                .args(vec!["run","-it","--network","none","-h",uname.trim(),"--name",uname.trim(),"ubuntu","bash"])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn().expect("Docker Failed to Start, Kicking!")
        };
        drop(docker.wait());
        println!("goodbye!");
    }
}

fn main() {

    let args = Args::parse();

/*    match args.key {
        Some(key) => {
            println!("key sent, checking");
            let rk = std::fs::read_to_string(".key");
            if let Ok(rkey) = rk {
                println!("key is valid, dropping to bash");
                if rkey == key {
                    let mut bsh = Command::new("sh")
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn().expect("Bash failed to start somehow");
                    drop(bsh.wait())
                }
            } else {
                eprintln!("Invalid key");
            }
        }
        None => {*/
            println!("loading docker");
            load_docker();
//        }
//    }
}
