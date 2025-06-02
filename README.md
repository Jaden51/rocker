# Rocker
Rocker is a CLI tool built in Rust used for containerization. 

### Prerequisites

Rocker can only be run on a Linux machine. If you don't have a Linux machine, a virtual machine will work.  

### Starting the CLI
Build the project from the root
```
cargo build
```

Run the `run` command to spawn a new process. This also sets up the Linux namespaces and isolates the filesystem using `chroot`. An example command is shown below. 

```
./target/debug/rocker run /bin/sh
```

### Motivation
Rocker is being built to learn how Docker and containerization in Linux works under the hood. Rust is being used to help with running Linux system calls and to learn a new language in systems programming.
