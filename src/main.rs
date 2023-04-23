use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use clap::Parser;

use personnel_api::{PersonnelError, PersonnelManager};

#[derive(Parser)]
struct Cli {
    template_path: PathBuf,
    db_path: PathBuf,
    params: String,
}

fn main() {
    let cli = Cli::parse();

    let params: HashMap<String, String> = cli
        .params
        .split(';')
        .flat_map(|p| p.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let report = create_report(&cli.template_path, &cli.db_path, params).unwrap();

    println!("{report}");
}

fn create_report(
    template_path: &Path,
    db_path: &Path,
    params: HashMap<String, String>,
) -> Result<String, PersonnelError> {
    let pm = PersonnelManager::new(db_path).unwrap();

    pm.make_report(params, read_to_string(template_path).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report() {
        let params: HashMap<String, String> = HashMap::from([("id".to_string(), "0".to_string())]);

        let report = create_report(
            Path::new("resources/test-template.xml"),
            Path::new("resources/test.sqlite"),
            params,
        ).unwrap();

        let report_example = read_to_string("resources/test-report.html").unwrap();

        assert_eq!(report, report_example)
    }
}
