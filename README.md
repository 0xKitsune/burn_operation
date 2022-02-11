<!-- PROJECT LOGO -->
<br />
<p align="center">
    <img src="assets/burn_operation.jpg" alt="Logo" width="394" height="360">
  </a>
  <h1 align="center">burn_operation</h1>
  <p align="center">

 
<br />


## Overview

A Rust based CLI that completely wipes a computer securely, at the speed of light. Nice and handy when you need to initiate "Burn Operation". This program overwrites files with random bytes, then deletes the file, effectively wiping the hard drive storage and making all of the data unrecoverable. Using `jwalk`, which enables parallelism through `rayon`, Rust is able to walk through a computer's directory tree at very high speeds.  

## How to use burn_operation

Instructions to install, add to path and execute. Ideally is a command in the terminal at any time that is just `burn_operation`. `-n=number_of_passes` flag to overwrite `n` times.



## Notes

Also add `dead_mans_switch`, it will prompt you to enter a password (which will hash a phrase, the password is not stored anywhere, just the hashed phrase), as well as a frequency to check in and how long it will wait for a response. Then it will prompt you for your password at that time interval, wait the response time, and if there is no response, it will initiate `burn_operation`.
