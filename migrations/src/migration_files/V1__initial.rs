use barrel::types;

pub fn migration() -> String {
    let mut m = barrel::Migration::new();
    println!("Applying: {}", file!());

    m.create_table("short_urls", |t| {
        t.add_column("id", types::varchar(6));
        t.add_column("url", types::varchar(255));

        t.set_primary_key(&["id"]);
    });

    m.make::<barrel::backend::Pg>()
}
