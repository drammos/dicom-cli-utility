use std::{
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use clap::Parser;
use dicom::{dictionary_std::tags, object::open_file};
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

    // Set Timer
    let start_time = Instant::now();

    // Counter(mutex) and console_lock(mutex)
    let counter = Arc::new(Mutex::new(0));
    let console_lock = Arc::new(Mutex::new(()));

    let mut handles = vec![];

    for entry in WalkDir::new(dicom_path).into_iter().filter_map(Result::ok) {
        let path = entry.path().to_path_buf();

        // Copy counter and console_loc(mutex)
        let thread_counter = Arc::clone(&counter);
        let thread_console_lock = Arc::clone(&console_lock);
        let thread_console_lock1 = Arc::clone(&console_lock);

        // Create new thread
        let handle = thread::spawn(move || {
            if path.is_file() {
                match extract_dicom(&path, thread_console_lock) {
                    Ok(_) => {
                        // Increase the counter
                        let mut counter = thread_counter.lock().unwrap();
                        *counter += 1;
                    }
                    Err(e) => e.handle_error(&path, thread_console_lock1),
                }
            }
        });
        handles.push(handle); // Add the handle in Vector
    }

    // Wait all threads to join
    for handle in handles {
        handle.join().unwrap();
    }
    let duration = start_time.elapsed();
    println!(
        "Total DICOM files processed: {} and time {:?}",
        counter.lock().unwrap(),
        duration
    );
}

/// extract_dicom file, read a file and Print PatientName and PatientID for this path
fn extract_dicom(path: &Path, console_lock: Arc<Mutex<()>>) -> Result<(), AppError> {
    let object = open_file(path)?;

    let patient_name = object.element(tags::PATIENT_NAME)?.to_str()?;
    let patient_id = object.element(tags::PATIENT_ID)?.to_str()?;

    // Lock the mutex
    let _lock = console_lock.lock().unwrap();
    println!("Found DICOM file:");
    println!("File Path: {}", path.display());
    println!("Patient Name: {}", patient_name);
    println!("Patient ID: {}", patient_id);
    println!("------------------------------------------- \n");
    Ok(())
}
