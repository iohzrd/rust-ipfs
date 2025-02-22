#[derive(Debug, clap::Parser)]
#[clap(name = "ipns")]
struct Opt {
    key: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::str::FromStr;

    use clap::Parser;
    use rust_ipfs::Ipfs;
    use rust_ipfs::IpfsPath;
    use rust_ipfs::UninitializedIpfsDefault as UninitializedIpfs;

    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    // Initialize the repo and start a daemon
    let ipfs: Ipfs = UninitializedIpfs::new()
        .with_default()
        .add_listening_addr("/ip4/0.0.0.0/tcp/0".parse()?)
        .with_mdns()
        .with_relay(true)
        .default_record_key_validator()
        .start()
        .await?;

    ipfs.default_bootstrap().await?;

    let ipns_path = IpfsPath::from_str(&opt.key)?;
    let path = ipfs.resolve_ipns(&ipns_path, false).await?;

    println!("{} resolves to {}", opt.key, path);

    Ok(())
}
