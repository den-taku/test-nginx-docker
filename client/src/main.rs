use rand::prelude::*;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let names = ["DenTaku", "dentaku", "studentaku", "testaku"];
    let mut rng = rand::thread_rng();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let name = names[rng.gen::<usize>() % names.len()];

        let body = reqwest::get(if cfg!(target_os = "linux") {
            // docker
            format!("http://server:7890/hello/{name}")
        } else {
            // local
            format!("http://127.0.0.1:7890/hello/{name}")
        })
        .await?
        .text()
        .await?;
        println!("body = {body:?}");
    }
}
