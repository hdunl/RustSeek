use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use ignore::{WalkBuilder, WalkState};
use rayon::prelude::*;
use zip::ZipArchive;

pub struct Search {
    rx: Box<dyn Iterator<Item = String>>,
    pub num_results: Arc<Mutex<usize>>,
    pub num_files_processed: Arc<Mutex<usize>>,
    pub duration: std::time::Duration,
}

impl Iterator for Search {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.rx.next();
        if item.is_some() {
            let mut num_results = self.num_results.lock().unwrap();
            *num_results += 1;
        }
        item
    }
}

impl Search {
    pub fn new(search_location: impl AsRef<Path>, search_input: Option<&str>, search_zip: bool) -> Self {
        let (tx, rx) = mpsc::channel::<String>();
        let search_input = search_input.map(|s| s.to_string());

        let location = search_location.as_ref().to_path_buf();
        let search_input_clone = search_input.clone();
        let tx_clone = tx.clone();
        let num_results = Arc::new(Mutex::new(0));
        let num_files_processed = Arc::new(Mutex::new(0));

        let start_time = Instant::now();

        let num_results_clone = Arc::clone(&num_results);
        let num_files_processed_clone = Arc::clone(&num_files_processed);
        thread::spawn(move || {
            search_in_directory(location, search_input_clone.as_deref(), tx_clone, num_results_clone, num_files_processed_clone);
        });

        if search_zip {
            let location = search_location.as_ref().to_path_buf();
            let search_input = search_input.clone();
            let tx = tx.clone();

            let num_results_clone = Arc::clone(&num_results);
            let num_files_processed_clone = Arc::clone(&num_files_processed);
            thread::spawn(move || {
                search_for_zip_files(location, search_input, tx, num_results_clone, num_files_processed_clone);
            });
        }

        let duration = start_time.elapsed();

        Self {
            rx: Box::new(rx.into_iter()),
            num_results,
            num_files_processed,
            duration,
        }
    }
}

fn search_in_directory(
    search_location: PathBuf,
    search_input: Option<&str>,
    tx: Sender<String>,
    num_results: Arc<Mutex<usize>>,
    num_files_processed: Arc<Mutex<usize>>,
) {
    let search_input = search_input.unwrap_or("").to_string();
    let walker = WalkBuilder::new(search_location)
        .threads(num_cpus::get())
        .build_parallel();

    walker.run(|| {
        let tx = tx.clone();
        let search_input = search_input.clone();
        let num_results = Arc::clone(&num_results);
        let num_files_processed = Arc::clone(&num_files_processed);

        Box::new(move |result| {
            if let Ok(entry) = result {
                {
                    let mut num_files_processed = num_files_processed.lock().unwrap();
                    *num_files_processed += 1;
                }
                if let Some(file_name) = entry.path().file_name().and_then(|n| n.to_str()) {
                    if file_name.contains(&search_input) {
                        tx.send(entry.path().display().to_string()).unwrap();
                        let mut num_results = num_results.lock().unwrap();
                        *num_results += 1;
                    }
                }
            }
            WalkState::Continue
        })
    });
}

fn search_for_zip_files(
    search_location: PathBuf,
    search_input: Option<String>,
    tx: Sender<String>,
    num_results: Arc<Mutex<usize>>,
    num_files_processed: Arc<Mutex<usize>>,
) {
    let walker = WalkBuilder::new(search_location)
        .threads(num_cpus::get())
        .build_parallel();

    walker.run(|| {
        let tx = tx.clone();
        let search_input = search_input.clone();
        let num_results = Arc::clone(&num_results);
        let num_files_processed = Arc::clone(&num_files_processed);

        Box::new(move |result| {
            if let Ok(entry) = result {
                {
                    let mut num_files_processed = num_files_processed.lock().unwrap();
                    *num_files_processed += 1;
                }
                if entry.path().extension().and_then(|e| e.to_str()) == Some("zip") {
                    let path = entry.path().to_path_buf();
                    let search_input = search_input.clone();
                    let tx = tx.clone();
                    let num_results = Arc::clone(&num_results);
                    let num_files_processed = Arc::clone(&num_files_processed);

                    thread::spawn(move || {
                        search_in_zip(path, search_input, tx, num_results, num_files_processed);
                    });
                }
            }
            WalkState::Continue
        })
    });
}

fn search_in_zip(
    zip_path: PathBuf,
    search_input: Option<String>,
    tx: Sender<String>,
    num_results: Arc<Mutex<usize>>,
    num_files_processed: Arc<Mutex<usize>>,
) {
    match fs::File::open(&zip_path) {
        Ok(file) => {
            let mut archive = match ZipArchive::new(file) {
                Ok(archive) => archive,
                Err(e) => {
                    eprintln!("Failed to read ZIP archive '{}': {:?}", zip_path.display(), e);
                    return;
                }
            };
            let files: Vec<(usize, String)> = (0..archive.len())
                .filter_map(|i| archive.by_index(i).ok().map(|file| (i, file.name().to_string())))
                .collect();

            files.into_par_iter().for_each(|(i, file_name)| {
                if search_input.as_ref().map_or(true, |input| file_name.contains(input)) {
                    tx.send(format!("{} in {}", file_name, zip_path.display())).unwrap();
                    let mut num_results = num_results.lock().unwrap();
                    *num_results += 1;
                }
                {
                    let mut num_files_processed = num_files_processed.lock().unwrap();
                    *num_files_processed += 1;
                }
            });
        }
        Err(e) => {
            eprintln!("Failed to open file '{}': {:?}", zip_path.display(), e);
        }
    }
}

// Author: Hayden Dunlap, Email: hdunlap936@gmail.com, hrdunlap@valdosta.edu, GitHub: github.com/hdunlap
