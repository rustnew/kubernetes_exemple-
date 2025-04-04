use kube::{Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::infer().await?;
    println!(
        "Connected to cluster at {:?}",
        config.cluster_url.host().unwrap()
    );
    Ok(())
}

