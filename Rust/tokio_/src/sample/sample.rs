use log::Level;
use tokio::{io::AsyncReadExt, time};

async fn sleeper() {
    log::info!("sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("awake");
}
async fn run() {
    sleeper().await;
    reader().await;
}
/**
 * the sleeper function never run
 */
async fn run_await() {
    sleeper();
    reader().await;
}
async fn run_spawn() {
    tokio::spawn(async {
        sleeper().await;
    });
    reader().await;
}
//similar to promise.all
async fn runJoint() {
    tokio::join!(sleeper(), reader());
}
async fn reader() {
    log::info!("Reading some data");
    let mut file = tokio::fs::File::open("src/sample.csv").await.unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).await.unwrap();
    log::info!("read big {} bytes", contents.len());
}
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

// fn main() {
//     println!("Hello, world!");
//     simple_logger::init_with_level(Level::Info).unwrap();
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let future = run();
//     rt.block_on(future);
// }
#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    run().await;
}
async fn reader_with_calculation() {
    reader().await;
    fib(40);
}
async fn reader_with_span() {
    reader().await;
    tokio::task::spawn_blocking(move || {
        log::info!("computing");
        fib(80);
        log::info!("computing finish");
    })
    .await
    .unwrap();
}
#[cfg(test)]
mod tests {
    use log::Level;

    #[test]
    fn reader_without_span() {
        println!("reader without span");
        simple_logger::init_with_level(Level::Info).unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = crate::reader_with_calculation();
        rt.block_on(future);
    }
    #[test]
    fn reader_with_span() {
        println!("reader with span");
        simple_logger::init_with_level(Level::Info).unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = crate::reader_with_span();
        rt.block_on(future);
    }
}
