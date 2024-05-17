use crate::config::Config;
use crate::repo;
use crate::utils;

pub async fn trasnlate(cfg: Config) {
    let mut db = repo::DBRepo::new(cfg.db.file_name);
    if let Err(err) = db.connect() {
        panic!("err: {}", err)
    }
    let translater = repo::Translater::new(cfg.translate_app);
    for book in cfg.book_list {
        translate_book(&db, &translater, &book).await;
    }
}

pub async fn translate_book(db: &repo::DBRepo, translater: &repo::Translater, book: &str) {
    let mut data = Vec::new();
    let book_result = db.get_vocabulary(book);
    match book_result {
        Ok(book) => {
            for word in book {
                let mut row = word.into_slice();
                let result = translater.translate(&word.spelling).await;
                if let Ok(resp) = result {
                    if resp.trans_result.len() != 0 {
                        let mut trans_text = Vec::new();
                        for res in resp.trans_result {
                            trans_text.push(res.dst.clone());
                        }
                        row.push(trans_text.join(";"));
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
        vec!["title", "spelling", "phonetic_uk", "translate_res"],
        data,
        format!("{}.xlsx", book),
    ) {
        Err(e) => println!("err: {}", e),
        Ok(_) => println!("export success"),
    };
}
