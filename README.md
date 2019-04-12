# cc2640r2f-hal

Rust `embedded-hal` hardware abstract layer implementation of TI CC2640R2F microcontroller

# State of this project

This project aim to port all bare-metal hardware abstracts, but in this layer, built-in TI-RTOS 
environment support is not to be included. 

It's under heavy development. If you managed to get a demoboard for this chip, try examples in
the `examples` folder. They might gotta work, but if not and you happened to know how to solve, 
please fire an issue or a pull request.

All kinds of contribution are welcomed. Before getting around the code, make sure you have a 
proper Rust installation by using `rustup` installer from official page, or run `rustup update` 
if already installed.
