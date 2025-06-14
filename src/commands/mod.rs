use std::fs::{create_dir_all, write};

pub mod list;
pub mod ps;
pub mod run;
pub mod stop;

const MEMORY_LIMIT: &str = "10000000"; // 10 MB

pub fn create_cgroup(pid: u32) {
    let cgroup_path = "/sys/fs/cgroup/rocker";

    create_dir_all(cgroup_path).expect("Failed to create cgroup");

    write(format!("{}/memory.max", cgroup_path), MEMORY_LIMIT).expect("Failed to set memory limit");

    write(format!("{}/cgroup.procs", cgroup_path), pid.to_string())
        .expect("Failed to add process to cgroup");
}
