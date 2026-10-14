use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.rc");
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-env-changed=KASPA_COLD_ZIG");
    println!("cargo:rerun-if-env-changed=KASPA_COLD_RC");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let resource;
    let status;
    if let Some(rc) = env::var_os("KASPA_COLD_RC") {
        resource = out.join("icon.res");
        status = Command::new(rc)
            .current_dir(root.join("assets"))
            .arg(format!("/fo{}", resource.display()))
            .arg("icon.rc")
            .status();
    } else if let Some(zig) = env::var_os("KASPA_COLD_ZIG") {
        resource = out.join("icon.res");
        status = Command::new(zig)
            .current_dir(root.join("assets"))
            .args(["rc", "/nologo"])
            .arg(format!("/fo{}", resource.display()))
            .arg("icon.rc")
            .status();
    } else if target.ends_with("msvc") {
        resource = out.join("icon.res");
        status = Command::new("rc")
            .current_dir(root.join("assets"))
            .arg("/nologo")
            .arg(format!("/fo{}", resource.display()))
            .arg("icon.rc")
            .status();
    } else {
        resource = out.join("icon.o");
        let compiler =
            env::var_os("WINDRES").unwrap_or_else(|| "x86_64-w64-mingw32-windres".into());
        status = Command::new(compiler)
            .current_dir(root.join("assets"))
            .args(["-i", "icon.rc", "-O", "coff", "-o"])
            .arg(&resource)
            .status();
    }
    assert!(
        status
            .expect("Windows icon resource compiler is required")
            .success(),
        "Windows icon resource compilation failed"
    );
    println!("cargo:rustc-link-arg-bin=kaspa-cold={}", resource.display());
}
