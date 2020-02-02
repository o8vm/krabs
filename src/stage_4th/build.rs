fn main() {
    cc::Build::new()
        .flag("-m32")
        .flag("--target=i586-unknown-none")
        .flag("-fno-common")
        .flag("-mno-sse")
        .flag("-fno-builtin")
        .flag("-ffreestanding")
        .file("c_src/huffman.c")
        .file("c_src/crctable.c")
        .file("c_src/randtable.c")
        .file("c_src/decompress.c")
        .file("c_src/bzlib.c")
        .compile("bz2");
}
