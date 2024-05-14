use crate::model::Vocabulary;
use rusqlite::{Connection, Error};

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

    pub fn connect(&mut self) -> Result<(), Error> {
        self.conn = Connection::open(self.db_file)?;
        Ok(())
    }

    pub fn get_vocabulary(conn: Connection, book: String) -> Result<Vec<Vocabulary>, Error> {
        Ok(vec![])
    }
}
