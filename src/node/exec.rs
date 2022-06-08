use crate::node::NodeInstall;
use std::process::Command;
use trauma::Error;

// #[cfg(target_os = "linux")]
pub fn up() {
    std::process::Command::new("clear").status().unwrap();
    println!("🚀 Up Lunes Node");

    match Command::new("systemctl")
        .arg("start")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error starting Lunes Node, {:?}", e),
        Ok(x) => x,
    };
    println!("✅ Done!");
}

// #[cfg(target_os = "linux")]
pub fn down() {
    std::process::Command::new("clear").status().unwrap();
    println!("🚨 Down Lunes Node");

    match Command::new("systemctl")
        .arg("stop")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error stopping Lunes Node, {:?}", e),
        Ok(x) => x,
    };
    println!("✅ Done!");
}

// #[cfg(target_os = "linux")]
pub fn logs() {
    std::process::Command::new("clear").status().unwrap();
    println!("📊 Logs Lunes Node");

    match Command::new("journalctl")
        .arg("-fu")
        .arg("lunesnode")
        .output()
    {
        Err(e) => panic!("Error show Lunes Node Logs, {:?}", e),
        Ok(x) => x,
    };
    println!("✅ Done!");
}

// #[cfg(target_os = "linux")]
pub fn status() {
    std::process::Command::new("clear").status().unwrap();
    println!("🌡  Status Lunes Node");

    match Command::new("systemctl")
        .arg("status")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error read status of Lunes Node, {:?}", e),
        Ok(x) => x,
    };
    println!("✅ Done!");
}

// #[cfg(target_os = "linux")]
pub fn restart() {
    std::process::Command::new("clear").status().unwrap();
    std::process::Command::new("clear").status().unwrap();
    println!("🔄 Restart Lunes Node");

    match Command::new("systemctl")
        .arg("restart")
        .arg("lunesnode.service")
        .output()
    {
        Err(e) => panic!("Error restarting Lunes Node, {:?}", e),
        Ok(x) => x,
    };
    println!("✅ Done!");
}

pub fn version() {
    println!("Comming Soon")
}

pub fn config() {
    println!("Comming Soon")
}

async fn downloading(version: String) -> Result<(), Error> {
    use std::path::PathBuf;
    use trauma::{download::Download, downloader::DownloaderBuilder};

    let url = format!(
        "https://github.com/lunes-platform/lunes-node/releases/download/{}/lunesnode-{}.jar",
        version, version
    );
    let downloads = vec![Download::try_from(url.as_str()).unwrap()];
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("."))
        .build();
    downloader.download(&downloads).await;
    Ok(())
}

fn move_files_to_opt(args: NodeInstall) {
    use super::utils::{mount_hocon, mount_service};
    use std::fs;

    mount_hocon(args.clone());
    mount_service();

    fs::create_dir_all("/opt/lunesnode").expect("failed in create dir /opt/lunesnode");
    fs::copy("lunesnode-0.0.8.jar", "/opt/lunesnode/lunesnode-0.0.8.jar")
        .expect("failed in move .jar");
    fs::remove_file("lunesnode-0.0.8.jar").expect("failed in remove tmp .jar");
}

// #[cfg(target_os = "linux")]
pub async fn install(args: NodeInstall) {
    match args.version {
        Some(ref v) => {
            std::process::Command::new("clear").status().unwrap();
            println!("⬇️ Downloading Lunes Node {}", v);
            downloading(v.to_string()).await;
            move_files_to_opt(args.clone());
            println!("✅ Done!");
            println!("🚀 Running `lunes node up` to start");
        }
        None => {
            std::process::Command::new("clear").status().unwrap();
            println!("⬇️  Downloading Lunes Node 0.0.8");
            downloading(String::from("0.0.8")).await.expect("");
            move_files_to_opt(args.clone());
            println!("✅ Done!");
            println!("🚀 Running `lunes node up` to start");
        }
    };
}
