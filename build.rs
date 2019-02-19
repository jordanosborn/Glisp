fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .include("lib/include")
        .include("/usr/local/include")
        .flag("-std=c++17")
        .compile(output);
}

fn main() {
    let src_files = vec!["lib/src/test.cpp"];
    build(src_files, "test");
}
