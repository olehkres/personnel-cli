use clap::Parser;
use personnel_api::{db::make_personchain, person::Person, report::make_report};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    person_id: u32,
    report_template: PathBuf,
    reference_template: PathBuf,
    output: PathBuf,
    db_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let person_chain: Vec<Person> = make_personchain(cli.person_id, cli.db_path.as_path()).unwrap();

    let document_template_path = cli.report_template.as_path();
    let reference_template_path = cli.reference_template.as_path();
    let output_file_path = cli.output.as_path();

    make_report(
        &person_chain,
        document_template_path,
        reference_template_path,
        output_file_path,
    )
    .unwrap();
}
