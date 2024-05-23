fn main() {
    let env: String = std::env::var("ENV_FROM_DOCKER")
        .unwrap_or("Can not find ENV from environment variables".to_string());
    println!("Hello, world!");
    println!("ENV_FROM_DOCKER: {}", env);
}
