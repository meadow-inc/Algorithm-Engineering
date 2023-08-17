In order to run the experiments, follow these steps:

1. Make sure you have Rust and Cargo installed on your system.

   Rust: https://www.rust-lang.org/tools/install

   Cargo: https://doc.rust-lang.org/cargo/getting-started/installation.html

2. Copy all instances you wish to test into: Exercise04/files/countries

3. Open a terminal in the "Exercise04" folder.

4. Compile the code in "main.rs" into an executable 
   by running: "cargo build --release" in the terminal

5. Run the executable by running "time ./target/release/Exercise04 files/countries/"

   - the "time" command is optional, yet helpful to measure the time
   - "./target/release/Exercise04" is the path of the executable
   - "files/countries/" is the folder where all .tsp files are located

   - the executable will compute all necessary run times, and should run in about 10-15 minutes
   - you will be informed in the console, which file is being processed.
   - China is expected to take about 2:30m per 2-Opt (so 5 minutes in total)

6. you can see the results in the "print.txt" file which will be located in the Exercise04 
   folder.
