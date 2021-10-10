use std::env;
use std::fs;
use std::path::Path;

fn remove_special_chars(fname: &str) -> String {
    const SPECIAL: &str = "-!\"#$%&\'()*+,/:;<=>?@[\\]^`{|}~ªº";
    let mut out = fname.replace(|x| x == '\u{20}' || x == '\u{3000}' || x == '-', "_"); // common or ideographic
    out = out.replace("__", "_");
    for c in SPECIAL.chars() {
        out = out.replace(c, "");
    }
    out
}

fn remove_zlib(fname: &str) -> String {
    const ZLIB: &str = " (z-lib.org)";
    let out = fname.replace(ZLIB, "");
    out
}

fn remove_dots_pdf(fname: &str) -> String {
    const PDF: &str = ".pdf";
    let split: Vec<&str> = fname.split(&PDF).collect();
    let mut prefix = split[0].replace(".", "");
    prefix.push_str(".pdf");
    prefix
}

fn normalize_fname(fname: &str) -> String {
    let mut out = remove_zlib(fname);
    out = remove_special_chars(&out);
    out = remove_dots_pdf(&out);
    out
}

fn main() {
    let paths = fs::read_dir(&Path::new(&env::current_dir().unwrap())).unwrap();

    let files = paths
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
                .filter(|x| x.ends_with(".pdf"))
        })
        .collect::<Vec<String>>();
    for f in &files {
        fs::rename(&f, normalize_fname(&f)).unwrap();
    }
}
