use migrations::embedded;
use rusqlite::Connection;

fn main() {
    let mut c = Connection::open("./db.sqlite").unwrap();

    let runner = embedded::migrations::runner();

    println!("Starting migrations...");

    let report = runner.run(&mut c).unwrap();

    for migration in report.applied_migrations() {
        println!(
            "Migration Applied - Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    println!("Finished migrations...");
}
