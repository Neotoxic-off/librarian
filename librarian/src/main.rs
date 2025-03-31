use clap::Parser;
use walkdir::{DirEntry, WalkDir};
use infer;
use log::{info, error};
use std::fs::File;
use std::io::{Read, BufReader};
use memchr::memmem;
use std::collections::HashMap;
use std::path::Path;
use rayon::prelude::*;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

mod arguments;
mod constants;
mod setup;

fn shannon_entropy(data: &[u8]) -> f64 {
    let mut counts: HashMap<u8, usize> = HashMap::new();

    for &byte in data {
        *counts.entry(byte).or_insert(0) += 1;
    }

    let len: f64 = data.len() as f64;

    counts.values()
        .map(|&count| {
            let probability: f64 = count as f64 / len;
            -probability * probability.log2()
        })
        .sum()
}

fn read_file_bytes(file_path: &str) -> Option<Vec<u8>> {
    let file: File = File::open(file_path).ok()?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer).ok()?;

    Some(buffer)
}

fn calculate_entropy(file_path: &str, threshold: f64) -> bool {
    if let Some(buffer) = read_file_bytes(file_path) {
        let entropy_value = shannon_entropy(&buffer);
        if entropy_value > threshold {
            error!("❌   High entropy: {}", get_filename(Path::new(file_path)).unwrap_or("Unknown".to_string()));
            return true;
        }
    }
 
    false
}

fn inspect_file(file_path: &str) -> bool {
    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return false
    };
    
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: [u8; 8192] = [0u8; 8192];
    let bytes_read: usize = match reader.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return false
    };

    matches!(infer::get(&buffer[..bytes_read]), Some(kind) if 
        kind.matcher_type() == infer::MatcherType::Audio || 
        kind.matcher_type() == infer::MatcherType::Video ||
        kind.matcher_type() == infer::MatcherType::Book ||
        kind.matcher_type() == infer::MatcherType::Doc ||
        kind.matcher_type() == infer::MatcherType::Font ||
        kind.matcher_type() == infer::MatcherType::Image ||
        kind.matcher_type() == infer::MatcherType::Video
    )
}

fn detect_script_patterns(file_path: &str) -> bool {
    let buffer = match read_file_bytes(file_path) {
        Some(bytes) => bytes,
        None => return false,
    };

    for &pattern in &constants::SCRIPT_PATTERNS {
        if memmem::find(&buffer, pattern).is_some() {
            error!("❌   Script found: {}", get_filename(Path::new(file_path)).unwrap_or("Unknown".to_string()));
            return true;
        }
    }
    
    false
}

fn get_filename(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|f| f.to_str())
        .map(|s| s.to_string())
}

fn process_file(file_path: &str, threshold: f64, detection_count: &Arc<AtomicUsize>) -> bool {
    let file_name = get_filename(Path::new(file_path)).unwrap_or("Unknown".to_string());

    if inspect_file(file_path) {
        if detect_script_patterns(file_path) || calculate_entropy(file_path, threshold) {
            detection_count.fetch_add(1, Ordering::Relaxed);
            info!("❌   {}", file_name);
            return true;
        }
        info!("✔️   {}", file_name);
    }
    false
}

fn scan_directory(folder: &str, threshold: f64, threads: usize) {
    info!("Scanning directory: {}", folder);

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let detection_count = Arc::new(AtomicUsize::new(0));
    let entries: Vec<DirEntry> = WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    thread_pool.install(|| {
        entries.par_iter().for_each(|entry| {
            let file_path = entry.path().to_string_lossy().to_string();
            let detection_count_clone = Arc::clone(&detection_count);
            process_file(&file_path, threshold, &detection_count_clone);
        });
    });

    let count = detection_count.load(Ordering::Relaxed);
    info!("Scan completed. Detections found: {}", count);
}

fn main() {
    setup::init_logger();

    let args: arguments::Arguments = arguments::Arguments::parse();
    info!("Settings: entropy threshold: {:.2} threads: {}", args.entropy_threshold, args.threads);

    scan_directory(&args.folder, args.entropy_threshold, args.threads);
}
