fn main() {
    cc::Build::new()
        .file("src/test_library.c")
        .compile("test_library");
}
