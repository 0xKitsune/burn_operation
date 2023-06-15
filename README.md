<!-- PROJECT LOGO -->
<br />
<p align="center">
    <img src="assets/burn_operation.jpg" alt="Logo" width="394" height="360">
  </a>
  <h1 align="center">burn_operation</h1>
  <p align="center">

 
<br />


## Overview

**DISCLAIMER**: This program is not fully tested and should not be assumed to work correctly until proper tests and validation are in place.

A Rust based CLI that completely wipes a computer securely, at the speed of light. Nice and handy when you need to initiate "Burn Operation". This program overwrites every file on your computer with random bytes, then deletes the file, effectively wiping the hard drive storage and making all of the data unrecoverable. Using `jwalk`, which enables parallelism through `rayon`, Rust is able to walk through a computer's directory tree, overwrite and delete files at very high speeds.  


## Installing
Make sure you have Rust installed. If not, install it with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. After installing Rust, enter the following command into your terminal to install Burn Operation:

```
cargo install burn_operation
```

## Permissions
By default, `burn_operation` wipes your entire computer which requires root access. You can either run the program with `sudo` or give access permissions to the burn_operation binary.

Giving permissions allows for quick and easy usage of the program without the need to input a password at runtime, however this can be a major security risk, so this is at your own risk. This can be useful if you need to wipe many computers on a network with one command or if your usecase requires that `burn_operation` is on a hair trigger. To give permissions to the binary, enter the following commands in your terminal below:

### Linux
`burn_operation`
```
BURN_OPERATION_BINARY=$(which burn_operation)
sudo chown root:root $BURN_OPERATION_BINARY
sudo chmod u+s $BURN_OPERATION_BINARY
```

`dead_mans_switch`
```
DEAD_MANS_SWITCH_BINARY=$(which dead_mans_switch)
sudo chown root:root $DEAD_MANS_SWITCH_BINARY
sudo chmod u+s $DEAD_MANS_SWITCH_BINARY
```

### Mac
`burn_operation`
```
BURN_OPERATION_BINARY=$(which burn_operation)
sudo chown root:wheel $BURN_OPERATION_BINARY
sudo chmod u+s $BURN_OPERATION_BINARY
```

`dead_mans_switch`
```
DEAD_MANS_SWITCH_BINARY=$(which dead_mans_switch)
sudo chown root:wheel $DEAD_MANS_SWITCH_BINARY
sudo chmod u+s $DEAD_MANS_SWITCH_BINARY
```

### Windows



## Usage
**IMPORTANT:** There is no safety on this by design. Once you execute the program, there is no going back. Do not use this command unless you are certain that you want to wipe your computer beyond recovery.

```

USAGE:
    burn_operation [OPTIONS]

OPTIONS:
    -d, --dead-mans-switch    Initializes a dead man's switch from a `dead_mans_switch.toml` file.

    -h, --help                Print help information

    -n, --n <N>               Number of iterations when wiping each file. [default: 25]

    -p, --path <PATH>         Path to file or directory to wipe. If no argument is provided, the
                              entire computer will be wiped. [default: /]
```

### `-n`
The `-n` flag indicates the number of iterations that the program will overwrite a file with random bytes. For example if a user inputs `burn_operation -n=25`, the program will overwrite each file with random bytes 25 times.


## Upcoming features

### Deadman's Switch
`dead_mans_switch` will wipe a computer after "x" time has passed without the user checking in. When burn_operation is run with this flag, you will be prompted you to enter a keyphrase (basically a password) of your choosing. This will be used to generate a hash. The hash will act as a security check, with burn_operation prompting you to enter your keyphrase to verify that you are the one who initiated the dead mans switch. You can also set how long the program should wait for your response after prompting you. After this time has elapsed, the program will automatically call `burn_operation`, wiping everything on the computer.
