use mini_redis::{client, Result};

 #[tokio::main]
 async fn main() -> Result<()> {
     // Open a ccnnection to the mini-redis address 
     let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key to "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get the key "hello"
    let result = client.get("hello").await?;

    println!("Got value: {:?} from server", result);

    Ok(())
 }
