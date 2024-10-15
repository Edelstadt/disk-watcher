use clap::Parser;
use env_logger;
use fs_extra::dir::get_size;
use log::{error, info, warn};
use rusqlite::{Connection, Result as SqlResult};
use std::time::Instant;
use walkdir::WalkDir;

// Args and env vars Parser
#[derive(Parser)]
#[command(
    version,
    about = "Directory size scraper",
    long_about = "Scrapes directory sizes and stores them in a SQLite database"
)]
pub struct Opts {
    #[arg(long, env, default_value = "/", help = "Path to watch")]
    watcher_path: String,
    #[arg(long, env, default_value = "2", help = "Depth to watch")]
    watcher_depth: usize,
    #[arg(
        long,
        env,
        default_value = "true",
        help = "Watch only the same filesystem as the 'start' path"
    )]
    watcher_same_fs: bool,
    #[arg(
        long,
        env,
        default_value = "10",
        help = "Number of directories to show"
    )]
    watcher_count: usize,
    #[arg(
        long,
        env,
        default_value = "/data/data.sqlite",
        help = "Path to SQLite database"
    )]
    db_path: String,
}

struct DirSize {
    directory: String,
    size: u64,
}

fn list_directories(path: &str, depth: usize, same_fs: bool) -> Vec<String> {
    let mut dirs = Vec::new();

    // Procházíme souborový systém
    for entry in WalkDir::new(path)
        .max_depth(depth)
        .same_file_system(same_fs)
        .into_iter()
    {
        match entry {
            Ok(e) => {
                if e.path().is_dir() {
                    let dir_str = e.path().display().to_string();
                    dirs.push(dir_str.clone());

                    info!("Directory found: {}", dir_str);
                }
            }
            Err(err) => {
                warn!("Error reading directory: {:?} - {}", err.path(), err);
            }
        }
    }

    // Hledání nadbytečných složek (podadresáře) pro smazání
    let mut dirs_to_delete = Vec::new();
    for dir in dirs.iter() {
        for dir2 in dirs.iter() {
            if dir != dir2 && dir2.starts_with(dir) {
                dirs_to_delete.push(dir.to_string());
            }
        }
    }

    // Odstraníme složky, které mají být smazány
    dirs.retain(|dir| !dirs_to_delete.contains(dir));

    // Vracíme seznam složek
    dirs
}

fn get_db_connection(db_path: &str) -> SqlResult<Connection> {
    let conn = Connection::open(db_path)?;
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS largest_folders (
            dir TEXT NOT NULL,
            size INTEGER NOT NULL
        )
        ",
        [],
    ) {
        Ok(_) => info!("Table created successfully"),
        Err(e) => error!("Error creating table: {}", e),
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS scrape_log (
            timestamp DEFAULT CURRENT_TIMESTAMP,
            elapsed TEXT NOT NULL
        )",
        [],
    ) {
        Ok(_) => info!("Table created successfully"),
        Err(e) => error!("Error creating table: {}", e),
    }
    Ok(conn)
}

fn main() {
    env_logger::init();
    let start = Instant::now();
    let opts = Opts::parse();

    let mut dir_sizes = Vec::new();
    let dirs = list_directories(&opts.watcher_path, opts.watcher_depth, opts.watcher_same_fs);
    for dir in dirs.iter() {
        // Bez použití unwrap, logování v případě chyby
        match get_size(dir) {
            Ok(size) => {
                let dir_size = DirSize {
                    directory: dir.to_string(),
                    size,
                };
                dir_sizes.push(dir_size);
            }
            Err(e) => {
                warn!("Failed to get size of directory {}: {}", dir, e);
            }
        }
    }

    dir_sizes.sort_by(|a, b| a.size.cmp(&b.size));
    let last_10 = dir_sizes.iter().rev().take(opts.watcher_count);

    let duration = start.elapsed();
    match get_db_connection(&opts.db_path) {
        Ok(conn) => {
            for dir_size in last_10 {
                match conn.execute(
                    "INSERT INTO largest_folders (dir, size) VALUES (?1, ?2)",
                    [&dir_size.directory, &dir_size.size.to_string()],
                ) {
                    Ok(_) => info!(
                        "Inserted directory {} with size {}",
                        dir_size.directory, dir_size.size
                    ),
                    Err(e) => error!("Error inserting directory: {}", e),
                }
            }
            match conn.execute(
                "INSERT INTO scrape_log (elapsed) VALUES (?1)",
                [&duration.as_secs_f64().to_string()],
            ) {
                Ok(_) => info!("Inserted scrape log"),
                Err(e) => error!("Error inserting scrape log: {}", e),
            }
        }
        Err(e) => error!("Error opening database: {}", e),
    }
}
