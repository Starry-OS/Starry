//! To allow for batch testing, we define a list of test cases that can be run in sequence.
extern crate alloc;
use alloc::boxed::Box;

#[allow(dead_code)]
const BUSYBOX_TESTCASES: &[&str] = &[
    "busybox sh busybox_testcode.sh",
    "busybox sh lua_testcode.sh",
];

#[allow(dead_code)]
const LIBC_TESTCASES: &[&str] = &["libctest_testcode.sh", "libc-bench"];

#[allow(dead_code)]
const NET_TESTCASES: &[&str] = &[
    "busybox sh netperf_testcode.sh",
    "busybox sh iperf_testcode.sh",
];

#[allow(dead_code)]
const LMBENCH_TESTCASES: &[&str] = &[
    "busybox echo latency measurements",
    "lmbench_all lat_syscall -P 1 null",
    "lmbench_all lat_syscall -P 1 read",
    "lmbench_all lat_syscall -P 1 write",
    "busybox mkdir -p /var/tmp",
    "busybox touch /var/tmp/lmbench",
    "lmbench_all lat_syscall -P 1 stat /var/tmp/lmbench",
    "lmbench_all lat_syscall -P 1 fstat /var/tmp/lmbench",
    "lmbench_all lat_syscall -P 1 open /var/tmp/lmbench",
    "lmbench_all lat_select -n 100 -P 1 file",
    "lmbench_all lat_sig -P 1 install",
    "lmbench_all lat_sig -P 1 catch",
    "lmbench_all lat_sig -P 1 prot lat_sig",
    "lmbench_all lat_pipe -P 1",
    "lmbench_all lat_proc -P 1 fork",
    "lmbench_all lat_proc -P 1 exec",
    "busybox cp hello /tmp",
    "lmbench_all lat_proc -P 1 shell",
    "lmbench_all lmdd label=\"File /var/tmp/XXX write bandwidth:\" of=/var/tmp/XXX move=1m fsync=1 print=3",
    "lmbench_all lat_pagefault -P 1 /var/tmp/XXX",
    "lmbench_all lat_mmap -P 1 512k /var/tmp/XXX",
    "busybox echo file system latency",
    "lmbench_all lat_fs /var/tmp",
    "busybox echo Bandwidth measurements",
    "lmbench_all bw_pipe -P 1",
    "lmbench_all bw_file_rd -P 1 512k io_only /var/tmp/XXX",
    "lmbench_all bw_file_rd -P 1 512k open2close /var/tmp/XXX",
    "lmbench_all bw_mmap_rd -P 1 512k mmap_only /var/tmp/XXX",
    "lmbench_all bw_mmap_rd -P 1 512k open2close /var/tmp/XXX",
    "busybox echo context switch overhead",
    "lmbench_all lat_ctx -P 1 -s 32 2 4 8 16 24 32 64 96",
];

#[allow(dead_code)]
const IOZONE_TESTCASES: &[&str] = &["busybox sh ./iozone_testcode.sh"];

/// FIXME: This test case is not working
#[allow(dead_code)]
const UNIX_TESTCASES: &[&str] = &["busybox sh ./unixbench_testcode.sh"];

#[allow(dead_code)]
const CYCLE_TESTCASES: &[&str] = &["busybox sh ./cyclictest_testcode.sh"];

#[allow(dead_code)]
const OTHER_TESTCASES: &[&str] = &["./dora up", "./dora start lebai_dataflow.yml"];

#[allow(unused)]
pub fn run_batch_testcases() {
    let testcase_str = option_env!("MONOLITHIC_TESTCASE").unwrap_or("");
    let mut test_iter = match testcase_str {
        "busybox" => Box::new(BUSYBOX_TESTCASES.iter()),
        "libc" => Box::new(LIBC_TESTCASES.iter()),
        "net" => Box::new(NET_TESTCASES.iter()),
        "lmbench" => Box::new(LMBENCH_TESTCASES.iter()),
        "iozone" => Box::new(IOZONE_TESTCASES.iter()),
        "unixbench" => Box::new(UNIX_TESTCASES.iter()),
        "cycle" => Box::new(CYCLE_TESTCASES.iter()),
        "other" => Box::new(OTHER_TESTCASES.iter()),
        _ => Box::new(OTHER_TESTCASES.iter()),
    };

    for testcase in test_iter {
        axstarry::run_testcase(testcase);
    }
}
