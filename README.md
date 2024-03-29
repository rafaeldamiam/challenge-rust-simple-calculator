# Challenge—8 languages in 5 days (RUST Code)

This project is a challenge that I set for myself, during my vacation period. The idea was to develop the same small project: a simple calculator that asks for the user for two numbers and one character (a math operator), then performs the operation using the user's input.


# First day (03/04) - GO and RUST
For the first day, I tried two languages for the first time: GO and RUST. GO was relatively easy, but RUST presented a bit of a challenge. I finished the GO code in the same day and RUST code on 03/06.  

**GO** code can be accessed by clicking [here](https://github.com/rafaeldamiam/challenge-go-simple-calculator).  

**RUST** code can be accessed by clicking [here](https://github.com/rafaeldamiam/challenge-rust-simple-calculator). 


# Second day (03/05) - Java
On the second day, I started coding with Java. Creating this simple code was easy, but after I finished it, I keep studying Java and tried for the first time use Spring Framework.  


**Java** code can be access clicking [here](https://github.com/rafaeldamiam/challenge-java-simple-calculator).  


# Third day (03/06) - Python
On the third day, I started coding with Python, it was easy because I already had an experience with that programming language. So, before I ended, I start to study Django, a web framework for python.  


**Python** code can be access clicking [here](https://github.com/rafaeldamiam/challenge-python-simple-calculator).  


# Fourth day (03/07) - C# and Shell Script
On the fourth day, I stated coding with C# and then moved to Shell Script. Writing in C# was my first experience with an “independent” code, without a teacher or a step-by-step course guiding me. About Shell Script, it isn't the first time I used. But it was a bit more complex then the order small code that was usual to me to write, for first time I used functions and case in a Shell Script code.  


**C#** code can be access clicking [here](https://github.com/rafaeldamiam/challenge-csharp-simple-calculator).  


**Shell Script**  code can be access clicking [here](https://github.com/rafaeldamiam/challenge-shellscript-simple-calculator).  


# Fifth day (03/08) - C++ and C
On the fifth day, I coded with C++ and C, it was my second time tring C++ but first time tring C. Both languages share the same concepts, so it wasn't difficult to write both codes.  


**C++** code can be access clicking [here](https://github.com/rafaeldamiam/challenge-cpp-simple-calculator).  


**C** code can be access clicking [here](https://github.com/rafaeldamiam/challenge-c-simple-calculator).  


# How to execute this RUST code
My enviroment was Ubuntu Mate 22.04, so I used apt to install this packages to be able to execute the RUST code (Curl probaly already was installed on your linux):

`apt-get install build-essential curl`

So I installed RUST through curl:  

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

While running the last command, will show an message of installation options, my code I execute in a stable-x86_64.

`1`

At end of execution, will show to you a command to configure your shell, run:

`source "$HOME/.cargo/env`

Then go to folder 'src' where the code is.  

# Using RUST Cargo:

In 'src' folder, use de command to build the code (it will create the folder target with the debug and binary file **IT'S ALREADY DONE IN THIS REPOSITORY** ):

`cargo build`

In this case, my application is already builded, so you can only access 'target/debug' folder and execute the binary, or use the command bellow:

`./target/debug/calculator`

Install cargo-watch to automatize the compilation process while you are coding:

`cargo install cargo-watch`

Then, must you run the command bellow and your code you be compiled and executed after each save you do in your 'main.rs' file:

`cargo watch -x run`