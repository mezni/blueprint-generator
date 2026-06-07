use std::process::Command;

const OSM_URL: &str = "https://download.geofabrik.de/africa/tunisia-latest.osm.pbf";
const DEFAULT_OUTPUT: &str = "/tmp/tunisia-latest.osm.pbf";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_path = args.get(1).cloned().unwrap_or_else(|| DEFAULT_OUTPUT.to_string());
    let db_url = std::env::var("PLATFORM_DB_URL").unwrap_or_else(|_| {
        let host = std::env::var("PLATFORM_DB_HOST").unwrap_or_else(|_| "localhost".into());
        let port = std::env::var("PLATFORM_DB_PORT").unwrap_or_else(|_| "5432".into());
        let user = std::env::var("PLATFORM_DB_USER").unwrap_or_else(|_| "platform_user".into());
        let pass = std::env::var("PLATFORM_DB_PASSWORD").unwrap_or_else(|_| "platform_pass".into());
        let name = std::env::var("PLATFORM_DB_NAME").unwrap_or_else(|_| "platform_db".into());
        format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, name)
    });

    println!("OSM Tunisia Import Tool");
    println!("=======================");
    println!("Download URL: {}", OSM_URL);
    println!("Output path: {}", output_path);
    println!("Database: {}", db_url);

    // Step 1: Download PBF
    println!("\n[1/2] Downloading Tunisia OSM data...");
    let status = Command::new("curl")
        .args([
            "-L",
            "-o",
            &output_path,
            OSM_URL,
        ])
        .status()
        .expect("Failed to execute curl. Is curl installed?");

    if !status.success() {
        eprintln!("ERROR: Download failed with status: {}", status);
        std::process::exit(1);
    }
    println!("Download complete: {}", output_path);

    // Step 2: Import via osm2pgsql
    println!("\n[2/2] Importing into PostGIS via osm2pgsql...");
    let import_status = Command::new("osm2pgsql")
        .args([
            "-d",
            &db_url,
            "-U",
            "platform_user",
            "-H",
            "localhost",
            "-P",
            "5432",
            "-S",
            "default.style",
            "--hstore",
            "--slim",
            "--drop",
            "--extra-attributes",
            "--prefix",
            "osm",
            "--output",
            "flex",
            &output_path,
        ])
        .status();

    match import_status {
        Ok(status) if status.success() => {
            println!("OSM import completed successfully!");
        }
        Ok(status) => {
            eprintln!("WARNING: osm2pgsql exited with status: {}", status);
            eprintln!("The import may be incomplete. Check the logs above.");
            eprintln!("You can manually run: osm2pgsql -d <db_url> -U platform_user {}", output_path);
        }
        Err(e) => {
            eprintln!("WARNING: Failed to run osm2pgsql: {}", e);
            eprintln!("Make sure osm2pgsql is installed.");
            eprintln!("You can manually import: osm2pgsql -d <db_url> -U platform_user {}", output_path);
        }
    }

    println!("\nOSM import tool finished.");
}
