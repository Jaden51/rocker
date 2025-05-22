use std::ffi::CString;
use std::fs::create_dir_all;

use clap::{Parser, Subcommand};
use nix::mount::{MsFlags, mount, umount};
use nix::sched::{CloneFlags, unshare};
use nix::sys::wait::waitpid;
use nix::unistd::{ForkResult, chroot, execvp, fork, sethostname, write};

#[derive(Parser)]
#[command(name = "rocker")]
#[command(about = "Docker in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { cmd_args: Vec<String> },
    Images,
    Ps,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { cmd_args } => {
            run_container(cmd_args);
        }
        Commands::Images => {
            list_images();
        }
        Commands::Ps => {
            show_containers();
        }
    }
}

fn run_container(cmd_args: &Vec<String>) {
    println!("command: {:?}", cmd_args);

    unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS)
        .expect("Failed to unshare namespaces");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Parent: created container process with PID: {}", child);

            waitpid(child, None).expect("Failed to wait for child");

            let proc_path = "./rootfs/proc";
            // if std::path::Path::new(proc_path).exists() {
            //     if let Err(e) = umount(proc_path) {
            //         eprintln!("Warning: Failed to unmount {}: {}", proc_path, e);
            //     }
            // }
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
                Ok(_) => {
                    write(std::io::stdout(), "setdir succeeded\n".as_bytes()).ok();
                }
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

            println!("child done");
        }
        Err(_) => println!("Fork failed"),
    }

    println!("done run");
}

fn list_images() {
    println!("Listing images");
}

fn show_containers() {
    println!("Showing containers");
}
