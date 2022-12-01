use migrations::embedded;

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    let db_url: &str = dotenv!("DATABASE_URL");

    println!("Connecting to db...");
    let mut client = postgres::Client::connect(db_url, postgres::NoTls)
        .expect("No connection could be established to DB");
    println!("Connected to db");

    let runner = embedded::migrations::runner();

    println!("Starting migrations...");

    let report = runner.run(&mut client).unwrap();

    for migration in report.applied_migrations() {
        println!(
            "Migration Applied - Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    println!("Finished migrations...");
}
