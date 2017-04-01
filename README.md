# PiBuzz
A lightweight Raspberry Pi agent to control a buzzer and read a Button. Many applications.

## Control and Feedback from your Pi

By default, a Raspberry lacks basic user interaction.  
You can log over the network, add various shields, but some simple actions are out of reach.  

We needed a simple way to interact with a Pi, and came with these constraints
* Minimal hardware
* Integration in available Pi cases
* Usable on Pi Zero
* Allow for several commands
* Provides some kind of feedback
* Can serve as alert

## The Hardware

Can't do simpler:  
* A push button
* An Buzzer

## The software

Unleash your creativity!  
Several agents can be used with this simple hardware.  
We will soon release the first one, and more complex one will follow.  

Tell us what you need!

### Shutdown / Reboot agent

The missing Pi Feature!

* A short press (1 sec) makes 1 beep then reboot the Pi
* A long press (3 sec) makes 3 beeps, one per second, then shutdown

### Feedback Agent

A versatile sound server

* A TCP server listens for commands and emits the corresponding sound.
* sound effets to define

### Alarm agent

A minimalist alarm clock. Wakes you up at programmed time, push button to stop.
