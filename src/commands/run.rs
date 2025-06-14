use nix::mount::{MsFlags, mount};
use nix::sched::{CloneFlags, unshare};
use nix::sys::wait::waitpid;
use nix::unistd::{ForkResult, chroot, execvp, fork, sethostname, write};
use std::ffi::CString;
use std::fs::{create_dir_all};
use uuid::Uuid;

use crate::commands::create_cgroup;
use crate::containers::state::{ContainerInfo, ContainerState};

pub fn run(cmd_args: &Vec<String>) {
    println!("command: {:?}", cmd_args);

    unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS)
        .expect("Failed to unshare namespaces");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Parent: created container process with PID: {}", child);

            create_cgroup(child.as_raw() as u32);

            save_metadata(child.as_raw() as u32, &cmd_args);

            waitpid(child, None).expect("Failed to wait for child");
        }
        Ok(ForkResult::Child) => {
            sethostname("default").expect("Failed to set hostname");

            create_dir_all("./rootfs/proc").unwrap();
            match mount(
                Some("proc"),
                "./rootfs/proc",
                Some("proc"),
                MsFlags::empty(),
                None::<&str>,
            ) {
                Ok(_) => {
                    write(std::io::stdout(), "Mounted /proc successfully\n".as_bytes()).ok();
                }
                Err(e) => {
                    let msg = format!("Mount failed: {}\n", e);
                    write(std::io::stderr(), msg.as_bytes()).ok();
                }
            }

            match chroot("./rootfs") {
                Ok(_) => {
                    write(std::io::stdout(), "chroot succeeded\n".as_bytes()).ok();
                }
                Err(e) => {
                    let msg = format!("chroot failed: {}\n", e);
                    write(std::io::stderr(), msg.as_bytes()).ok();
                }
            }

            match std::env::set_current_dir("/") {
                Ok(_) => {}
                Err(e) => {
                    let msg = format!("setdir failed: {}\n", e);
                    write(std::io::stderr(), msg.as_bytes()).ok();
                }
            }
            let cmd = CString::new(cmd_args[0].as_str()).unwrap();
            let args: Vec<CString> = cmd_args
                .iter()
                .map(|s| CString::new(s.as_str()).unwrap())
                .collect();

            execvp(&cmd, &args).expect("Failed to execute command");
        }
        Err(_) => println!("Fork failed"),
    }
}

pub fn save_metadata(child_pid: u32, command: &Vec<String>) {
    let container_id = Uuid::new_v4().to_string();
    let info = ContainerInfo {
        id: container_id.clone(),
        pid: child_pid,
        command: command.clone(),
        state: ContainerState::Running,
    };
    std::fs::create_dir_all("./containers").unwrap();
    let meta_path = format!("./containers/{}.json", container_id);
    std::fs::write(meta_path, serde_json::to_string(&info).unwrap()).unwrap();
}
