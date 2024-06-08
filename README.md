# DCS without a joystick
The aim of this program is to make it as convenient and smooth as possible to play DCS World even without a throttle or joystick, by letting you use what hardware you already have laying around. The program should take care of things, remove pain points, and make it possible to use evil hardware combos that you normally can't.

We are creating this program because at the time of writing, you need to do a bunch of stuff and have a bunch of programs running to functionally fly in DCS on mouse and keyboard, and even after that, there are problems which we could not find existing software to fix.
### Planned features:
1. The program is to allow users to use virtually any combination of the following, through vjoy controllers in DCS being administrated by this application.
	- Keyboard
	- Mouse
	- Any controller (Xbox, Ps)
	- Any Arduino
	- Any Raspberry Pi

2. While flying in DCS with keyboard and mouse (like god intended) we observed some annoying things with how DCS handles the mouse, namely that there is a "menu" mouse position and an "in-game" mouse position, with the menu position overriding the in-game one whenever a menu is opened, leading to sharp jerks in joystick position whenever the comms menu is opened, or whenever the escape menu is closed.
   This program is to carefully watch the mouse, and when esc menu is closed (or comms menu opened), it is to try to move the mouse to exactly where it was when the menu was opened, undoing the jerk basically instantly.

3. We can not predict any combination of any setup of arduino/raspberry pi hardware, it would be near impossible to design something simple to work with every hardware combo, so instead a capable interface is to be provided. The task of wiring up/using what they have available, and making their arduino/raspberry pi send the proper signals, is to be left to the user.

4. Someone may want to use a joystick as a 4 way hat or an 8 way hat, the program is to support taking axes like from a joystick and using them like this, as well as vice versa.
