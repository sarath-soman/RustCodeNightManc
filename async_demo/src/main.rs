#[tokio::main]
async fn main() {
    let a = tokio::spawn(async { "Hello"});
    let b = tokio::spawn(async { "Rust" });
    println!("{} {}", a.await.unwrap(), b.await.unwrap())
}
