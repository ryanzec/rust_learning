use tokio::io::AsyncReadExt;
use tokio::time::{Duration, sleep};
use tokio::{fs::File, io, task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    return Ok(contents);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let task = task::spawn(async {
        let results = read_file("async_sleep_test.txt").await;

        match results {
            Ok(contents) => println!("file contents: {contents}"),
            Err(error) => println!("error reading file: {error}"),
        }
    });

    // once the task is created, it automatically starts as generally 1-3 (sometimes 4) will print before the async
    // task prints the file contents (depending on the machine's hardware)
    println!("task1 has started");
    println!("1");
    sleep(Duration::from_nanos(1)).await;
    println!("2");
    sleep(Duration::from_nanos(1)).await;
    println!("3");
    sleep(Duration::from_nanos(1)).await;
    println!("4");
    sleep(Duration::from_nanos(1)).await;
    println!("5");
    sleep(Duration::from_nanos(1)).await;
    println!("6");
    sleep(Duration::from_nanos(1)).await;
    println!("7");

    // other code can execute however if you get to a point when you need to wait for the task
    // to finish, you use just do this
    task.await?;

    println!("task1 has finished");

    return Ok(());
}
