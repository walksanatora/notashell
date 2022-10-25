use std::process::{Command, Stdio};

fn main() {

    let name = Command::new("whoami").output().expect("Unnable to get Username, Kicking!").stdout.into_boxed_slice();
    let uname = String::from_utf8_lossy(&name);
    
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
            Command::new("docker")
                .args(vec!["attach",uname.trim()])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn().expect("Docker Failed to Start, Kicking!")
        } else {
            Command::new("docker")
                .args(vec!["start","-ai",uname.trim()])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn().expect("Docker Failed to Start, Kicking!")
        }
        
    } else {
        Command::new("docker")
            .args(vec!["run","-it","--network","none","-h",uname.trim(),"--name",uname.trim(),"ubuntu","bash"])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn().expect("Docker Failed to Start, Kicking!")
    };
    drop(docker.wait());
    println!("goodbye!")
}
