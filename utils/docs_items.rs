#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [dependencies]
//! scraper = "0.21"
//! ```
//! Shows stats about the documented items.
//
// IMPROVE: use table format output

use scraper::{Html, Selector};
use std::{
    env,
    error::Error,
    fs,
    io::{self, BufWriter, Write},
    path::Path,
    process::Command,
};

const TARGET_DIR: &str = env!("CARGO_TARGET_DIR");
const IDS: [&str; 8] =
    ["structs", "enums", "macros", "statics", "traits", "functions", "attributes", "derives"];

fn main() -> Result<(), Box<dyn Error>> {
    compile_docs()?; // MAYBE:IMPROVE make optional

    header("All public items, except dependencies:");
    process_document("all/index.html", false)?;

    header(
        "All items, with exported dependencies (except core, alloc & std)
and crate-private items (if compiled with --document-private-items)",
    );
    process_document("all.html", true)?;
    Ok(())
}

/// Compiles documentation running this command: `cargo +nightly nd -F _docsrs`
fn compile_docs() -> Result<(), Box<dyn Error>> {
    let status = Command::new("cargo").args(["+nightly", "nd", "-F", "_docsrs"]).status()?;
    if !status.success() {
        eprintln!("Failed to compile documentation.");
        return Err("Documentation compilation failed".into());
    }
    Ok(())
}

/// Processes the document at the given subpath, with optional `dep_split`
fn process_document(subpath: &str, dep_split: bool) -> Result<(), Box<dyn Error>> {
    let doc_path = format!("{}/doc/devela/{}", TARGET_DIR, subpath);
    if !Path::new(&doc_path).exists() {
        println!("Documentation file not found: {}", doc_path);
        return Ok(());
    }

    // Load and parse the HTML content
    let html_content = fs::read_to_string(&doc_path)?;
    let document = Html::parse_document(&html_content);

    let hrefs = if dep_split {
        count_list_items_with_split(&document)
    } else {
        count_list_items_no_split(&document)
    };

    // Save hrefs to a text file
    let output_file_path = format!("items-{}.txt", subpath.replace("/", "_"));
    save_href_list(&output_file_path, &hrefs)?;
    Ok(())
}

/// Counts `<li>` elements with `_dep` prefix split.
fn count_list_items_with_split(document: &Html) -> Vec<String> {
    let anchor_selector = Selector::parse("a").unwrap();
    let mut hrefs = Vec::new();

    let (mut total, mut dep_total, mut non_dep_total) = (0, 0, 0);
    for id in &IDS {
        let selector_str = format!(r#"#{id} + ul li"#);
        let selector = Selector::parse(&selector_str).unwrap();

        let (mut count, mut dep_count, mut non_dep_count) = (0, 0, 0);

        for li_element in document.select(&selector) {
            if let Some(anchor) = li_element.select(&anchor_selector).next() {
                if let Some(href) = anchor.value().attr("href") {
                    // filter out std re-exported items
                    if href.starts_with("_dep/_core")
                        || href.starts_with("_dep/_alloc")
                        || href.starts_with("_dep/_std")
                        || href.starts_with("_info/examples")
                        || href.starts_with("all/")
                    {
                        continue;
                    }
                    count += 1;

                    hrefs.push(href.to_string());
                    if href.starts_with("_dep") {
                        dep_count += 1;
                    } else {
                        non_dep_count += 1;
                    }
                }
            }
        }
        total += count;
        dep_total += dep_count;
        non_dep_total += non_dep_count;
        println!("{id}: {count} (crate: {non_dep_count}, deps: {dep_count})");
    }
    let total = format!["items: {total} (crate: {non_dep_total}, deps: {dep_total})"];
    println!["{}\n{total}", "-".repeat(total.len())];

    hrefs
}

/// Counts `<li>` elements without `_dep` prefix split.
fn count_list_items_no_split(document: &Html) -> Vec<String> {
    let mut hrefs = Vec::new();

    let mut total = 0;
    for id in &IDS {
        let selector_str = format!(r#"#{id} + ul li"#);
        let selector = Selector::parse(&selector_str).unwrap();
        let mut count = 0;
        for li_element in document.select(&selector) {
            count += 1;
            if let Some(anchor) = li_element.select(&Selector::parse("a").unwrap()).next() {
                if let Some(href) = anchor.value().attr("href") {
                    hrefs.push(href.to_string());
                }
            }
        }
        println!("{id}: {count}");
        total += count;
    }
    let total = format!["items: {total}"];
    println!["{}\n{total}", "-".repeat(total.len())];

    hrefs
}

/// Saves a list of hrefs to a text file.
fn save_href_list(file_path: &str, hrefs: &[String]) -> io::Result<()> {
    let file = fs::File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    for href in hrefs {
        writeln!(writer, "{}", href)?;
    }
    Ok(())
}

/// Prints a header.
fn header(string: &str) {
    let max_len = string.lines().map(|line| line.len()).max().unwrap_or(0);
    println!("\n{string}\n{}", "-".repeat(max_len));
}
