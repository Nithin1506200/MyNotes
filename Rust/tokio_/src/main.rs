use log::Level;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // simple_logger::init_with_level(Level::Info).unwrap();
    let listner = TcpListener::bind("localhost:8080").await.unwrap();
    // log::info!();

    let (mut socket, _addr) = listner.accept().await.unwrap();
    loop {
        let mut buffer = [0u8; 1024];
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_log() {
        simple_logger::init_with_level(log::Level::Info).unwrap();
        log::info!("hello");
        log::warn!("this is warn");
        log::error!("erro");
        log::debug!("fjklj")
    }
}
