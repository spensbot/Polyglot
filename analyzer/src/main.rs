#![allow(dead_code)]
mod tools;
mod word_db;
mod texts;
mod user_level;

fn main() {
    let db = word_db::WordDb::load();
    // db.test();

    let texts = texts::Texts::load(&db);
    texts.test(&db);

    // let string = "before.

    // “Come";

    // for c in string.chars() {
    //     println!("{:x}", c as u32);
    // }
}
