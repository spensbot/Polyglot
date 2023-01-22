mod db;
mod hanzi_db;
mod texts;
mod _exploration;


fn main() {
    // let db = db::Db::load().unwrap();

    let hanzi_db = hanzi_db::HanziDb::load().unwrap();
    let db = db::Db::load().unwrap();

    db.print()

    // _exploration::explore();
}
