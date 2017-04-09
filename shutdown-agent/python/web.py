#!/usr/bin/env python

'''
PiBuzz. btnagent lib
Webhook emitter test and
Audio feedback Webservice

@actuino
'''

import btnagent
import requests
import httpserver

'''
TODO: This will be moved to a config file.
'''
# URL of webhook (the unicorn display client on this Pi)
WEBHOOK_URL = 'http://127.0.0.1:8080/command'
# Payload to post to the webhook when button clicked
WEBHOOK_LOAD = {"Command":"NextPage"}

def send_webhook_json(dic):
	requests.post(WEBHOOK_URL, json=dic)

@btnagent.click()
def btn_click(duration):
	print("Click",duration)
	try:
		send_webhook_json(WEBHOOK_LOAD)
	except Exception as e:
		print "Webhook Error ", str(e)

@btnagent.long_click()
def btn_long_click(duration):
	print("Long Click",duration)
	if duration == 1:
		print "Should reboot"
	if duration == 2:
		print "Shoud shutdown"
		
@httpserver.buzzer_command()
def bililip(cmd):
	print "Got Buzzer Command",cmd
	btnagent.play_morse('... --- ...',0.1);

if __name__ == '__main__':
	# Audio feedback listens on 8081 by default
	# Send commands as json to /buzzer
	httpserver.start('127.0.0.1',8081)
	btnagent.loop()
