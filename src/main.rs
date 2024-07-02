mod serde_type;
use tokio::{self, main};

#[main]
async fn main() {
    get_data().await;
}

async fn get_data() {
    let url = format!("https://www.jw.org/pl/biblioteka/biblia/biblia-wydanie-do-studium/ksiegi/json/html/42001024-42080051",);

    println!("test");
    let res = reqwest::get(url)
        .await
        .expect("o co tu chodzi???")
        .json::<serde_type::Root>()
        .await
        .expect("o co tu chodzi2???");

    let data = serde_json::to_string(&res).expect("no musi to być");
    println!("test kurde {}", data);

    println!("test kurde");
}
