use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        // Create a new task for each inbound socket.
        // The socket is moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut database = HashMap::new();

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(command) => {
                database.insert(command.key().to_string(), command.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(command) => {
                if let Some(value) = database.get(command.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            command => panic!("unimplemented {:?}", command),
        };

        // Write response back to client
        connection.write_frame(&response).await.unwrap();
    }
}
