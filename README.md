# Using PicoBricks OLED (SSD 1306) with embassy

## Description
Trivial example of controlling the *OLED* part of the PicoBricks board
using a Raspberry Pi Pico W. This should work with a non W Pico.

The program will display ascii letter (upper / lower case) using the alphabetical order
as row and column.
For example:

> abcdedfghijklmnop
> bcdedfghijklmnopq
> cdedfghijklmnopqr

The delay between each new character is controlled by the *potentiometer* (maximum 999 ms) (*GPIO26*). To display the delay, press the *button* (*GPIO10*) and keep it down. The LED will be lit up (*GPIO7*).

## Misc
*Cargo run* is configured to flash the RPI directly when pressing *BOOTSEL* while
resetting (I don't have a probe right now).

This code is inspired by embassy I2C examples and the terminal_i2c example from the ssd1306 crate.
