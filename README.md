# Get started #
Get started by going to www.rustup.rs , copy the link and run it from terminal in your home directory.

Follow instructions (stable version, press 1 and enter), then run source $HOME/.cargo/env

Type rustc --version and see you have a working version.

Run cargo new <name of your project> and you get a starter project for rust.

To compile and run the starter project (with debug symbols), simply cd -> <name of your project> and type cargo run. This will compile and execute the code.

To only compile run cargo build from same location as above (with debug symbols)

To compile a release build without debug symbols, cargo build --release

To run an executable, just run ./<executable>


## TIPS ##
To reduce binary size, run strip <executable> . This will remove all symbols from the object file reducing it's size

If you are using vscode. install extension rust-analyzer (and accept the language package it asks you to install together with it)