use std::path::Path;

use clap::Parser;
use dicom::object::open_file;
use walkdir::WalkDir;

use dicom_cli_utility::error::AppError;

#[derive(Parser)]
struct Args {
    path: String,
}

/// MAIN function
fn main() {
    // Parsing the args for input
    let args = Args::parse();
    let dicom_path = Path::new(&args.path);

    // Counter for Dicom Files
    let mut counter: i32 = 0;

    for entry in WalkDir::new(dicom_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        match extract_dicom(path) {
            Ok(()) => {
                counter += 1;
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    println!("Total DICOM files processed: {}", counter);
}

/// extract_dicom file, read a file and get PatientName and PatientID
fn extract_dicom(path: &Path) -> Result<(), AppError> {
    let object = open_file(path)?;

    let patient_name = object.element_by_name("PatientName")?.to_str()?;
    let patient_id = object.element_by_name("PatientID")?.to_str()?;

    println!("Found DICOM file:");
    println!("File Path: {}", path.display());
    println!("Patient Name: {}", patient_name);
    println!("Patient ID: {}", patient_id);
    println!("-------------------------------------------\n");
    Ok(())
}
