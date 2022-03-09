use std::io;

pub const RESPONSE: &'static [u8] =
    b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello world!";

pub const ADDRESS: &'static str = "127.0.0.1:8080";

fn main() -> io::Result<()> {
    tokio_uring::start(async {
        let listener = tokio_uring::net::TcpListener::bind(ADDRESS.parse().unwrap())?;

        loop {
            let (stream, _) = listener.accept().await?;

            tokio_uring::spawn(async move {
                let (result, _) = stream.write(RESPONSE).await;

                if let Err(err) = result {
                    eprintln!("Client connection failed: {}", err);
                }
            });
        }
    })
}
