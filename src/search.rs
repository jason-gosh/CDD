use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

use crate::shared::hex_console_colors;

pub fn find_directory(prefix: &str) -> Option<HashSet<String>> {
    let dirs = found_and_write_similarities_dirs(prefix);
    if dirs.len() == 0 {
        None
    } else {
        Some(dirs)
    }
}

pub fn list_directories(print_dirs: bool) -> io::Result<HashSet<String>> {
    let mut list_directories = HashSet::new();
    let current_dir = std::env::current_dir()?;

    if print_dirs {
        println!("Available directories {}:", current_dir.display());
    }

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let path_str: String = path
                .file_name()
                .map(|os_str| os_str.to_string_lossy().into_owned())
                .unwrap_or_default();
            list_directories.insert(path_str.clone());
            if print_dirs {
                print!(
                    "{} {}/ {}",
                    hex_console_colors::CYAN,
                    path_str,
                    hex_console_colors::RESET
                );
            }
        }
    }
    println!();

    Ok(list_directories)
}

pub fn count_frecuency_chars(text: &str) -> HashMap<char, usize> {
    let mut frequency_map = HashMap::new();

    for c in text.chars() {
        if !c.is_whitespace() {
            *frequency_map.entry(c).or_insert(0) += 1;
        }
    }

    let mut inputs: Vec<_> = frequency_map.clone().into_iter().collect();
    inputs.sort_by_key(|&(caracter, _)| caracter);

    let result: Vec<String> = inputs
        .into_iter()
        .map(|(char, count)| format!("{}: {}", char, count))
        .collect();

    println!("{}", result.join(", "));

    frequency_map
}

// used on count_frecuency_chars_unsensible
fn normalize_chars(c: char) -> char {
    let char_lower = c.to_lowercase().next().unwrap_or(c);

    match char_lower {
        'á' | 'à' | 'ä' | 'â' => 'a',
        'é' | 'è' | 'ë' | 'ê' => 'e',
        'í' | 'ì' | 'ï' | 'î' => 'i',
        'ó' | 'ò' | 'ö' | 'ô' => 'o',
        'ú' | 'ù' | 'ü' | 'û' => 'u',
        _ => char_lower,
    }
}

fn to_hashset_chars_unsensible(text: &str) -> HashSet<char> {
    let mut chars_set = HashSet::new();

    for c in text.chars() {
        if !c.is_whitespace() && !c.is_ascii_punctuation() {
            let c_normalized = normalize_chars(c);
            chars_set.insert(c_normalized);
        }
    }

    chars_set
}

fn is_text_similar_to_set(text: &str, set: HashSet<char>) -> bool {
    let t_size = text.chars().count();
    let s_size = set.len();

    if s_size > t_size {
        false
    } else {
        let is_similar = set.iter().all(|c| text.contains(*c));
        is_similar
    }
}

fn is_text_exactly_to_set(text: &str, set: HashSet<char>) -> bool{
    let t_size = text.chars().count();
    let s_size = set.len();
    

    if s_size == t_size && set.iter().all(|c| text.contains(*c)) {
        true
    } else {
        false
    }
}

pub fn found_and_write_similarities_dirs(prefix: &str) -> HashSet<String>{
    let mut similar_dirs = HashSet::new();
    let directories = list_directories(true);
    let pharase = to_hashset_chars_unsensible(prefix);
    match directories {
        Ok(dirs) => {
            println!("directories founds.");
            for dir in dirs {
                if is_text_similar_to_set(&dir, pharase.clone()) {
                    similar_dirs.insert(dir.clone());
                }
                if is_text_exactly_to_set(&dir, pharase.clone()){
                    similar_dirs.clear();
                    similar_dirs.insert(dir.clone());
                    
                    return similar_dirs;
                }
            }
        }
        _ => {}
    }
    similar_dirs
}
