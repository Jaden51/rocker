# Rocker
Rocker is a command-line tool written in Rust that provides containerization by isolating processes using Linux namespaces and cgroups. 

## Prerequisites

Rocker requires a Linux environment to run, as it relies on Linux-specific features like namespaces and cgroups. If you are not using a Linux machine, you can use a Linux virtual machine to run the program. 

## Commands

### run

```
rocker run <command>
```

Runs a new isolated container process with the specified command. 

### stop

```
rocker stop <container_id>
```

Stops the specified container process with the given id.

### ps

```
rocker ps
```

List currently running containers.

## Motivation
Rocker is a learning project designed to explore how Docker and Linux containerization work at a low level. It uses Rust to interact with Linux system calls, providing hands-on experience with both container internals and systems programming in Rust.
