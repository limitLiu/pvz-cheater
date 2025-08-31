fn main() {
  cc::Build::new().file("../process/src/main.c").compile("main");
  // println!("cargo:rustc-link-lib=dylib=System");
  // println!("cargo:rustc-link-lib=dylib=proc");
}
