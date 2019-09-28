use cc;

fn main() {
    cc::Build::new()
        .file("src/microwave.c")
        .warnings(true)
        .compile("microwave");
}
