<!-- PROJECT LOGO -->
<br />
<p align="center">
    <img src="assets/burn_operation.jpg" alt="Logo" width="394" height="360">
  </a>
  <h1 align="center">burn_operation</h1>
  <p align="center">

 
<br />


## Overview

A Rust based CLI that completely wipes a computer securely, at the speed of light. Nice and handy when you need to initiate "Burn Operation". This program overwrites every file on your computer with random bytes, then deletes the file, effectively wiping the hard drive storage and making all of the data unrecoverable. Using `jwalk`, which enables parallelism through `rayon`, Rust is able to walk through a computer's directory tree, overwrite and delete files at very high speeds.  

## How to use burn_operation

Installing burn_operation takes two simple steps.

First, clone the burn_operation git repo.

```
git clone https://github.com/0xKitsune/burn_operation.git && cd burn_operation

```

Make sure you have Rust installed. If you don't, run this command to install it: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

Once you have rust installed, run the following command.

```
cargo install --path .
```

Now you are all set up and can run burn_operation from anywhere in your terminal. **IMPORTANT:** There is no safety on this by design. Once you execute the program, there is no going back. Do not use this command unless you are certain that you want to wipe your computer beyond recovery.

With all of that out of the way, to run burn_operation to completely wipe your computer, run the following command from anywhere in your terminal.

```
burn_operation
```


## Upcoming features

### burn_operation -n
The `-n` flag indicates the number of passes that the programw will overwrite the file with random bytes. For example if a user inputs `burn_operation -n=5`, the program will overwrite each file with random bytes 5 times.


### Deadman's Switch
`dead_mans_switch` will wipe a computer after "x" time has passed without the user checking in. When burn_operation is run with this flag, you will be prompted you to enter a keyphrase (basically a password) of your choosing. This will be used to generate a hash. The hash will act as a security check, with burn_operation prompting you to enter your keyphrase to verify that you are the one who initiated the dead mans switch. You can also set how long the program should wait for your response after prompting you. After this time has elapsed, the program will automatically call `burn_operation`, wiping everything on the computer.
