use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split},
    net::TcpStream,
};

const SERVER_ADDRESS: &str = "localhost:8080";

#[tokio::main]
async fn main() {
    println!("Connecting to {}", SERVER_ADDRESS);

    match TcpStream::connect(SERVER_ADDRESS).await {
        Ok(stream) => {
            // connected
            println!(
                "Connected to {}:{}",
                stream.local_addr().unwrap().ip(),
                stream.local_addr().unwrap().port()
            );

            // Split the stream into read and write halves
            let (read_half, write_half) = split(stream);
            let mut reader = BufReader::new(read_half);
            let mut writer = write_half;
            
            // Spawn task for reading server messages
            let read_handle = tokio::spawn(async move {
                let mut line = String::new();
                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => { // EOF (Ctrl +D)
                            println!("Goodbye!");
                            break;
                        }
                        Ok(_) => {
                            // Send message to server
                            print!("{}", line);
                        }
                        Err(e) => {
                            eprintln!("Error reading from stdin: {}", e);
                            break;
                        }
                    }
                }
            });

            // Spawn task for handling user input
            let write_handle = tokio::spawn(async move {
                let stdin = tokio::io::stdin();
                let mut stdin_reader = BufReader::new(stdin);
                let mut line = String::new();

                loop {
                    line.clear();
                    match stdin_reader.read_line(&mut line).await {
                        Ok(0) => { // EOF (Ctrl +D)
                            println!("Goodbye!");
                            break;
                        }
                        Ok(_) => {
                            // Send message to server
                            if let Err(e) = writer.write_all(line.as_bytes()).await {
                                eprint!("Error writing to server: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            eprint!("Error reading input: {}", e);
                            break;
                        }
                    }
                }
            });

            // Wait for both tasks to complete
            let _ = tokio::join!(read_handle, write_handle);

        }
        Err(e) => {
            println!("Failed to connect to {}: {}", SERVER_ADDRESS, e);
        }
    }
}
