async fn foo() -> i32 {
    42
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("{}", foo().await);
}
