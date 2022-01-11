// use std::{env, process::Command};
// use which::which;

fn main() {
  // let mut wasm_pack = which("wasm-pack");

  // if wasm_pack.is_err() {
  //   println!("cargo:warn=`wasm-pack` crate not found, installing (this may take a while)");

  //   Command::new(
  //     which("cargo").expect("Expected `cargo` to be installed on your system, and in $PATH")
  //   )
  //   .arg("install")
  //   .arg("wasm-pack")
  //   .spawn()
  //   .unwrap()
  //   .wait()
  //   .unwrap();

  //   wasm_pack = which("wasm-pack");
  // }

  // let wasm_pack = wasm_pack.unwrap();

  // Command::new(wasm_pack)
  //   .arg("build")
  //   .arg("--target")
  //   .arg("web")
  //   .spawn()
  //   .unwrap()
  //   .wait()
  //   .unwrap();
}
