use anyhow::Result;

mod poc;

#[tokio::main]
async fn main() -> Result<()> {
    poc::docker_container::insert_demo().await?;
    println!("docker db access success");
    poc::atlas::tutorial().await?;
    println!("atlas db access success");

    Ok(())
}
