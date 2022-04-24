use std::process::Command;

pub fn up() {
    println!("Up Lunes Node");

    match Command::new("systemclt")
        .arg("start")
        .arg("lunesnode.service")
        .status()
    {
        Err(e) => panic!("Error starting Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}

pub fn down() {
    println!("Down lunes node");

    match Command::new("systemclt")
        .arg("stop")
        .arg("lunesnode.service")
        .status()
    {
        Err(e) => panic!("Error stopping Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}

pub fn logs() {
    println!("Show logs of lunes node");

    match Command::new("journalctl")
        .arg("-fu")
        .arg("lunesnode")
        .status()
    {
        Err(e) => panic!("Error show Lunes Node Logs, {:?}", e),
        Ok(x) => x,
    };
}

pub fn status() {
    match Command::new("systemclt")
        .arg("status")
        .arg("lunesnode.service")
        .status()
    {
        Err(e) => panic!("Error read status of Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}

pub fn version() {
    println!("version: 1.1.1")
}

pub fn config() {
    println!("pass your config")
}

pub fn install(version: Option<String>) {
    match version {
        Some(v) => println!("Installing your lunes node version: {}", v),
        None => println!("Installing your lunes node version: latest"),
    }
}

pub fn restart() {
    println!("Restart Lunes Node");

    match Command::new("systemclt")
        .arg("restart")
        .arg("lunesnode.service")
        .status()
    {
        Err(e) => panic!("Error restarting Lunes Node, {:?}", e),
        Ok(x) => x,
    };
}