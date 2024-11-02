use sqlist::Sqlite;
mod stack;
mod sqlist;
fn main() {
    let mut sqlite: Sqlite<i32> = Sqlite::new();
    sqlite.max_length=1;
    sqlite.push(1);
    sqlite.push(2);
    sqlite.push(3);
    sqlite.delete_by_index(0);
    println!("{}\n {:#?}", sqlite.max_length, sqlite);
}