use nix::sys::signal::{SIGKILL, kill};
use nix::unistd::Pid;
use std::fs;

use crate::containers::state::{ContainerInfo, ContainerState};

pub fn stop(container_id: &str) {
    let meta_path = format!("./containers/{}.json", container_id);

    let serialized_info = match fs::read_to_string(&meta_path) {
        Ok(content) => {
            content
        }
        Err(e) => {
            println!("Container not found: {}", e);
            return;
        }
    };

    let mut info: ContainerInfo = match serde_json::from_str(&serialized_info) {
        Ok(info) => info,
        Err(e) => {
            println!("Failed to parse container info: {}", e);
            return;
        }
    };
	println!("{}", info.pid);
    match kill(Pid::from_raw(info.pid as i32), SIGKILL) {
		Ok(_) => {
			println!("container stopped");
		}
		Err(e) => {
			println!("{}", e);
			return;
		}
	}
    info.state = ContainerState::Stopped;
    fs::write(meta_path, serde_json::to_string(&info).unwrap()).unwrap();
}
