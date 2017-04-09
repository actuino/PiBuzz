#!/usr/bin/env python

'''
PiBuzz. btnagent lib
Example of usage

@actuino
'''

import btnagent

@btnagent.click()
def btn_click(duration):
	print("Click",duration)

@btnagent.long_click()
def btn_long_click(duration):
	print("Long Click",duration)
	if duration == 1:
		print "Should reboot"
	if duration == 2:
		print "Shoud shutdown"

if __name__ == '__main__': 
	btnagent.loop()
