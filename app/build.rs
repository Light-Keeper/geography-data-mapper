fn main() {
    println!("cargo:rerun-if-changed=./priv/migrations");
}
