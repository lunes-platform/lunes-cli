use crate::node::NodeInstall;

pub fn mount_hocon(args: NodeInstall) {
    let (chain, full_node) = match args.chain {
        1 => ("MAINNET", "5.196.155.34:7770"),
        _ => ("TESTNET", "5.196.155.46:7770"),
    };

    std::fs::write(
        "/opt/lunesnode/lunesnode.conf",
        format!(
            r#"lunes {{
    directory = "./blockchain"
    blockchain.type = "{}"
    network.known-peers = [{}]
    wallet.password = "{}"
    wallet.seed = "{}"
}}"#,
            chain, full_node, args.password, args.seed_base58
        ),
    )
    .expect("failed in create config file for lunesnode");
}

pub fn mount_service() {
    std::fs::write(
        "/etc/systemd/system/lunesnode.service",
        r#"[Unit]
Description = Full Node of Lunes Blockchain
After = network.target

[Service]
WorkingDirectory = /opt/lunesnode/
ExecStart = /usr/bin/java -jar lunesnode.jar lunesnode.conf
Restart = always
RestartSec = 30s
StandardOutput = journal
StandardError = journal
SyslogIdentifier = lunesnode

[Install]
WantedBy = multi-user.target
"#,
    )
    .expect("failed in create service file");
}
