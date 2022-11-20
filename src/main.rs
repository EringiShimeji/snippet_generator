use std::{
    fs::{self, File},
    io::Write,
};

use snippet_generator::snippet::SnippetManager;

fn main() {
    let mut m = SnippetManager::default();
    m.gen(20, 20);
    let output = m.json();

    fs::create_dir("./cpp").unwrap();
    fs::create_dir("./cpp/.vscode").unwrap();
    let mut f = File::create("./cpp/.vscode/cpp.code-snippets").unwrap();

    f.write_all(output.as_bytes()).unwrap();
}
