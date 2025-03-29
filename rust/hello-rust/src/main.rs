use std::error::Error;



#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let x = async_main().await.unwrap();
    println!("x: {}", x);
}


pub async fn async_main() -> Result<i32, Box<dyn Error>>{
    println!("Hello, world Async!");
    let x = 42;
    Ok(x)
}

