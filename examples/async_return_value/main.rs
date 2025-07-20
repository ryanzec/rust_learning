use tokio::io::AsyncReadExt;
use tokio::{fs::File, io, task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    return Ok(contents);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let task_handle = task::spawn(async {
        let results = read_file("async_simple.txt")
            .await
            .expect("could not read file");

        return results;
    });

    // once the task is created, it automatically starts
    println!("task1 has started");

    let t = task_handle.await?;

    // other code can execute however if you get to a point when you need to wait for the task
    // to finish, you use task::block_on();
    // task::block_on(task);

    println!("{t}");
    println!("task1 has finished");

    return Ok(());
}
