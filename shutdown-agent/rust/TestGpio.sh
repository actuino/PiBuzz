#!/bin/sh

# ls /sys/class/gpio/gpio7 => Error

# To access to a pin it first has to be exported 
# Create a GPIO file access
echo 7 > /sys/class/gpio/export
sleep 1
# ls /sys/class/gpio/gpio7 => OK

# Once a pin is exported you need to set its 'direction'
# Configure the Pin Direction (In/Out)
# Lecture (in), Ecriture (out)
echo out > /sys/class/gpio/gpio7/direction

# Write / Read value on a pin
# non-zero value is interpreted as a high signal
echo 1 > /sys/class/gpio/gpio7/value
cat /sys/class/gpio/gpio7/value
sleep 1
# zero value is interpreted as a low signal
echo 0 > /sys/class/gpio/gpio7/value
cat /sys/class/gpio/gpio7/value

# Delete the created GPIO
# echo 7 > /sys/class/gpio/unexport