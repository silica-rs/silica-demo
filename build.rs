fn main() {
    println!("cargo:rerun-if-changed=../cortex-m3.json");
    println!("cargo:rerun-if-changed=layout.ld");
}
