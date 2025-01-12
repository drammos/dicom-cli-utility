# DICOM CLI Utility

A command-line utility to traverse directories and process DICOM files in parallel using multi-threading. The utility reads the patient information (such as Patient Name and Patient ID) from each DICOM file and prints the results to the console. It also gracefully handles errors and issues that may occur during file processing.

## 📑 Features:
- **Multi-threaded processing**: Efficiently handles DICOM file processing by leveraging multi-threading.
- **Error handling**: Gracefully handles invalid, non-DICOM files, access errors, and missing data elements.
- **Directory traversal**: Recursively traverses a directory to find `.dcm` files.
- **Concurrency safety**: Uses `Mutex` to safely handle the shared counter and console output between threads.

## 🛠️ Requirements:
- **Rust** (latest stable version)
- **Cargo** (Rust's package manager and build system)

## 🔧 Project Setup


To build the project, follow these steps:

1. **Install Rust**:
   If you haven't installed Rust yet, you can do so by following the instructions [here](https://www.rust-lang.org/tools/install).

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/drammos/dicom-cli-utility.git
   cd dicom-cli-utility

### Build the Project

In the project directory, run:

```bash
cargo build
```

### How to Use
Once the project is built, you can use it from the command line.

Command Syntax:
```bash
cargo run -- <path-to-dicom-directory>
```

#### Example Usage:

```bash
cargo run -- ./files
```

Where ./files is the path to the directory containing your .dcm files. The utility will recursively traverse all subdirectories and process any .dcm files it encounters.

### Parameters:
path: A required parameter specifying the directory path to start the DICOM file search from.

## How It Works

1. **Accepts Directory Path**: The utility starts by accepting the directory path as an argument.
2. **Traverses the Directory**: It recursively traverses the directory tree, looking for files with the `.dcm` extension.
3. **Processes Valid DICOM Files**:
    - For each valid DICOM file, it attempts to read the Patient Name and Patient ID from the file.
    - The extracted information is then printed to the console.
4. **Error Handling**: Errors such as invalid or unreadable DICOM files are handled gracefully, and informative error messages are provided.
5. **Concurrent Processing**: The processing is done concurrently using multiple threads to speed up the operation, especially when handling a large number of files.


## Error Handling

The utility uses custom error types defined in `AppError` and handles errors in the following cases:

- **ReadError**: When a file is not a valid DICOM file.
- **AccessError**: When there is an issue accessing a specific part of a DICOM file.
- **ConvertValueError**: When the value cannot be converted.
- **AccessByNameError**: When there is an issue accessing data by name in the DICOM file.

## 👤 Authors
[Rammos Dimitrios](https://github.com/drammos)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

