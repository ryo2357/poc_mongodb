use anyhow::Result;

mod poc;

#[tokio::main]
async fn main() -> Result<()> {
    poc::usage_examples::multi_insert::insert().await?;
    Ok(())
}

#[allow(dead_code)]
async fn tutorial_verify() -> Result<()> {
    poc::docker_container::insert_demo().await?;
    println!("docker db access success");
    poc::atlas::tutorial().await?;
    println!("atlas db access success");
    Ok(())
}
