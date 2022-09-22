use migrations::migrations::runner;
use rusqlite::Connection;

fn main() {
    let mut c = Connection::open("./db.sqlite").unwrap();

    let runner = runner();

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
