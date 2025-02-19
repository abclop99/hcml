//! Tests

use html_compare_rs::{assert_html_eq, HtmlCompareOptions};
use std::{fs, path::PathBuf};

use hcml::parser::parse;

fn test_example(hcml_file: PathBuf, html_file: PathBuf) {
    let document = fs::read_to_string(hcml_file).expect("Error opening HCML file");

    let html = parse(&document)
        .expect("Error converting HCML to HTML")
        .to_string();

    let ground_truth = fs::read_to_string(html_file).expect("Error opening HTML file");

    assert_html_eq!(html, ground_truth, HtmlCompareOptions {
        ignore_comments: false,
        ..Default::default()
    });
}

/// Test if example-1 works properly
#[test]
fn example_1() {
    test_example(
        PathBuf::from("example_files/example-1.hcml"),
        PathBuf::from("example_files/example-1.html"),
    );
}
