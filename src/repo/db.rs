use crate::model::Vocabulary;
use rusqlite::{Connection, Result};

pub struct DBRepo {
    db_file: String,
    conn: Option<Connection>,
}

impl DBRepo {
    pub fn new(db_file: String) -> Self {
        DBRepo {
            db_file,
            conn: None,
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        let path = self.db_file.clone();
        println!("connect db file path: {}", path);
        self.conn = Some(Connection::open(path)?);
        Ok(())
    }

    pub fn get_vocabulary(&self, book: &str) -> Result<Vec<Vocabulary>> {
        let mut stmt = if let Some(conn) = &self.conn {
            conn.prepare("
SELECT title,spelling,phonetic_uk
FROM VOC_TB
INNER JOIN (
SELECT title,voc_origin_id,chapter_origin_id,`order`
FROM BK_VOC_TB V
INNER JOIN BK_CHAPTER_TB C ON V.chapter_origin_id= C.id AND V.book_origin_id IN (
SELECT origin_id
FROM BK_TB
WHERE name = ?)) AS tmp ON VOC_TB.origin_id=tmp.voc_origin_id
ORDER BY `order`;
")?
        } else {
            return Ok(vec![]);
        };
        let vocabulary_list_iterator = stmt.query_map([book], |row| {
            Ok(Vocabulary {
                title: row.get(0)?,
                spelling: row.get(1)?,
                phonetic_uk: row.get(2)?,
            })
        })?;
        let mut vocabulary_list = Vec::new();
        for v in vocabulary_list_iterator {
            vocabulary_list.push(v?)
        }
        Ok(vocabulary_list)
    }
}
