use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    fn gen_c_to_rust_bindings(in_file: &str, out_file: &str) {
        println!("cargo:rerun-if-changed={in_file}");

        let allow_types = ["tm", "jmp_buf"];
        let mut builder = bindgen::Builder::default()
            .header(in_file)
            .clang_arg("-I./include")
            .derive_default(true)
            .size_t_is_usize(false)
            .use_core();
        for ty in allow_types {
            builder = builder.allowlist_type(ty);
        }

        builder
            .generate()
            .expect("Unable to generate c->rust bindings")
            .write_to_file(out_file)
            .expect("Couldn't write bindings!");
    }

    gen_c_to_rust_bindings("ctypes.h", "src/libctypes_gen.rs");

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "aarch64" {
        aarch64_vfp_compile();
    }
}

fn aarch64_vfp_compile() {
    // 获取当前 crate 输出目录
    let out_dir = env::var("OUT_DIR").unwrap();
    // 指定汇编文件路径
    let asm_file = PathBuf::from("src/vfp_setjmp.S");
    let asm_out_file = PathBuf::from(&out_dir).join("vfp_setjmp.o");

    // 编译汇编文件，增加 target-feature 选项
    let status = Command::new("clang")
        .args(&[
            "-c", asm_file.to_str().unwrap(),
            "-o", asm_out_file.to_str().unwrap(),
            "-target", "aarch64-unknown-none",
            "-mfpu=neon"
        ])
        .status()
        .expect("failed to execute clang");
    assert!(status.success(), "clang failed to compile assembly file");

    // 打包对象文件为静态库
    let lib_out_file = PathBuf::from(&out_dir).join("libvfp_setjmp.a");
    let status = Command::new("ar")
        .args(&["crus", lib_out_file.to_str().unwrap(), asm_out_file.to_str().unwrap()])
        .status()
        .expect("failed to execute ar");
    assert!(status.success(), "ar failed to create static library");

    // 指示 rustc 链接器链接汇编对象文件
    println!("cargo:rerun-if-changed=src/vfp_setjmp.S");
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=vfp_setjmp");
}
