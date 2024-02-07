# Using Picobricks OLED (SSD 1306) with embassy

Trivial example of controlling the OLED part of the picobrick using a Raspberry
PI Pico W. This should work with a non W Pico.

Cargo is configured to flash the RPI directly when pressing BOOTSEL while
resetting (I don't have a probe right now).

This code is inspired by embassy I2C examples and the terminal_i2c example from the ssd1306
crate.
