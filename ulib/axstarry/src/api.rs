use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use linux_syscall_api::read_file;
/// To get the environment variables of the application
///
/// # TODO
/// Now the environment variables are hard coded, we need to read the file "/etc/environment" to get the environment variables
pub fn get_envs() -> Vec<String> {
    // Const string for environment variables
    let mut envs:Vec<String> = vec![
        "SHLVL=1".into(),
        "PWD=/".into(),
        "GCC_EXEC_PREFIX=/riscv64-linux-musl-native/bin/../lib/gcc/".into(),
        "COLLECT_GCC=./riscv64-linux-musl-native/bin/riscv64-linux-musl-gcc".into(),
        "COLLECT_LTO_WRAPPER=/riscv64-linux-musl-native/bin/../libexec/gcc/riscv64-linux-musl/11.2.1/lto-wrapper".into(),
        "COLLECT_GCC_OPTIONS='-march=rv64gc' '-mabi=lp64d' '-march=rv64imafdc' '-dumpdir' 'a.'".into(),
        "LIBRARY_PATH=/lib/".into(),
        "LD_LIBRARY_PATH=/lib/".into(),
        "LD_DEBUG=files".into(),
    ];
    // read the file "/etc/environment"
    // if exist, then append the content to envs
    // else set the environment variable to default value
    if let Some(environment_vars) = read_file("/etc/environment") {
        envs.push(environment_vars);
    } else {
        envs.push("PATH=/usr/sbin:/usr/bin:/sbin:/bin".into());
    }
    envs
}
/// To run a testcase with the given name, which will be used in initproc
///
/// The environment variables are hard coded, we need to read the file "/etc/environment" to get the environment variables
pub fn run_testcase(testcase: &str) {
    let _ = linux_syscall_api::run_testcase(testcase, get_envs());
}

/// To print a string to the console
pub fn println(s: &str) {
    axlog::ax_println!("{}", s);
}
