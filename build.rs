extern crate cc;

use std::env;
use std::process::Command;

use std::fs::remove_file;
use std::io::Write;
use tempfile;

fn main() {
    let mut builder = cc::Build::new();
    for i in &[
        "size_t",
        "unsigned int",
        "unsigned long",
        "unsigned long long",
    ] {
        let def_name = format!("SIZEOF_{}", i.to_uppercase().replace(" ", "_"));
        builder.define(&def_name, Some(check_native_size(i).as_str()));
    }
    builder.define("SECONDARY_DJW", Some("1"));
    builder.define("SECONDARY_FGK", Some("1"));
    builder.define("EXTERNAL_COMPRESSION", Some("0"));
    builder.define("XD3_USE_LARGEFILE64", Some("1"));

    #[cfg(windows)]
    builder.define("XD3_WIN32", Some("1"));
    builder.define("SHELL_TESTS", Some("0"));

    builder.include("xdelta3/xdelta3");
    builder.file("xdelta3/xdelta3/xdelta3.c").compile("xdelta3");
}

fn check_native_size(name: &str) -> String {
    let builder = cc::Build::new();
    let out_dir = env::var("OUT_DIR").unwrap();
    let compiler = builder.get_compiler();
    let mut compile = Command::new(compiler.path().as_os_str());
    let test_code = format!("#include <stdint.h>\n#include <stdio.h>\nint main() {{printf(\"%lu\", sizeof({})); return 0;}}\n", name);
    let test_binary_fn = format!("{}/test-{}", out_dir, name);
    let mut test_source = tempfile::Builder::new()
        .suffix(".c")
        .tempfile_in(out_dir)
        .expect("Error creating test compile files");
    let test_source_fn = test_source.path().to_string_lossy();
    compile
        .args(compiler.args())
        .args(&[&test_source_fn, "-o", test_binary_fn.as_str()]);
    test_source
        .write_all(test_code.as_bytes())
        .expect("Error writing test compile files");
    compile.output().expect("Error compiling test source");
    compile = Command::new(&test_binary_fn);
    let output = compile
        .output()
        .expect("Error executing test binary")
        .stdout;
    let output = String::from_utf8(output).expect("Error converting Unicode sequence");
    remove_file(test_binary_fn).ok();
    return output;
}
