use crate::node::NodeInstall;
use std::process::Command;

use super::utils::{mount_hocon, mount_service};

#[cfg(target_os = "linux")]
pub fn up() {
    println!("Up Lunes Node");

    match Command::new("systemctl")
        .arg("start")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error starting Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}

#[cfg(target_os = "linux")]
pub fn down() {
    println!("Down Lunes Node");

    match Command::new("systemctl")
        .arg("stop")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error stopping Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}

#[cfg(target_os = "linux")]
pub fn logs() {
    println!("Logs Lunes Node");

    match Command::new("journalctl")
        .arg("-fu")
        .arg("lunesnode")
        .output()
    {
        Err(e) => panic!("Error show Lunes Node Logs, {:?}", e),
        Ok(x) => x,
    };
}

#[cfg(target_os = "linux")]
pub fn status() {
    println!("Status Lunes Node");

    match Command::new("systemctl")
        .arg("status")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error read status of Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}

#[cfg(target_os = "linux")]
pub fn restart() {
    println!("Restart Lunes Node");

    match Command::new("systemctl")
        .arg("restart")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error restarting Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}

pub fn version() {
    println!("Comming Soon")
}

pub fn config() {
    println!("Comming Soon")
}

#[cfg(target_os = "linux")]
pub fn install(args: NodeInstall) {
    std::fs::create_dir("/opt/lunesnode/");
    mount_hocon(args.clone());
    mount_service();

    match args.version {
        Some(v) => match v.as_str() {
            "0.0.7" => {
                println!("Downloading Lunes Node ...");
                Command::new("wget")
                .args(["-O", "/opt/lunesnode/lunesnode.jar"])
                .arg("https://github.com/lunes-platform/lunes-node/releases/download/0.0.7/lunesnode.jar")
                .output()
            },
            "0.1.0" => {
                println!("Downloading Lunes Node ...");
                Command::new("wget")
                .args(["-O", "/opt/lunesnode/lunesnode.jar"])
                .arg("https://github.com/lunes-platform/lunes-node/releases/download/0.1.0/lunesnode.jar")
                .output()
            },
            _ => {
                println!("Downloading Lunes Node ...");
                Command::new("wget")
                .args(["-O", "/opt/lunesnode/lunesnode.jar"])
                .arg("https://github.com/lunes-platform/lunes-node/releases/download/0.1.0/lunesnode.jar")
                .output()
            }
        },
        None => {
            println!("Downloading Lunes Node ...");
            Command::new("wget")
            .args(["-O", "/opt/lunesnode/lunesnode.jar"])
            .arg("https://github.com/lunes-platform/full-node/releases/download/0.1.0/lunesnode.jar")
            .output()
        }
    };
}
