mod serde_type;
use tokio::{self, main};

#[main]
async fn main() {
    get_data().await;
}

async fn get_data() {
    let _url_test = format!("https://b.jw-cdn.org/apis/pub-media/GETPUBMEDIALINKS?output=json&pub=osg&fileformat=MP3%2CAAC&alllangs=0&langwritten=PL&txtCMSLang=PL");

    let range = "01001003";
    let url = format!(
        "https://www.jw.org/pl/biblioteka/biblia/biblia-wydanie-do-studium/ksiegi/json/html/{}",
        &range
    );

    let test_url = "https://jsonplaceholder.typicode.com/todos/1";

    println!("test");
    dbg!(&url);
    let res = reqwest::get(test_url).await.unwrap();

    // .json::<serde_type::Book>()
    // .await;
    dbg!(&res);

    println!("test kurde");
    // match res {
    //     Ok(..) => {
    //         println!("res OK");
    //     }
    //     Err(e) => {
    //         println!("Nie no kupa");
    //         panic!("Nie udało się pobrać piosenek. Err: {}", e)
    //     }
    // }
}
