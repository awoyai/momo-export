mod config;
mod model;
mod repo;
mod utils;

#[tokio::main]
async fn main() {
    let book_list = vec![
        "译林版牛津英语七年级上册",
        "译林版牛津英语七年级下册",
        "译林版牛津英语八年级上册",
        "译林版牛津英语八年级下册",
        "译林版牛津英语九年级上册",
        "译林版牛津英语九年级下册",
    ];
    let cfg = config::Config::init();
    println!("config: {:?}", cfg);
    let mut db = repo::DBRepo::new(cfg.db.file_name);
    if let Err(err) = db.connect() {
        panic!("err: {}", err)
    }
    let translater = repo::Translater::new(cfg.translate_app);
    for book in book_list {
        translate_book(&db, &translater, book).await;
    }
}

async fn translate_book(db: &repo::DBRepo, translater: &repo::Translater, book: &str) {
    let mut data = Vec::new();
    let book_result = db.get_vocabulary(book);
    match book_result {
        Ok(book) => {
            for word in book {
                let mut row = word.into_slice();
                let result = translater.translate(&word.spelling).await;
                if let Ok(resp) = result {
                    if resp.trans_result.len() != 0 {
                        row.push(resp.trans_result[0].dst.clone());
                    }
                }
                data.push(row);
            }
        }
        Err(err) => {
            println!("get_vocabulary err: {}", err)
        }
    };
    match utils::excel::save_data(
        vec!["title", "spelling", "phonetic_uk"],
        data,
        format!("{}.xlsx", book),
    ) {
        Err(e) => println!("err: {}", e),
        Ok(_) => println!("export success"),
    };
}
