#!/usr/bin/env python2.7
from emokit import emotiv
import gevent
from gevent.socket import wait_read

import sys


class State:
  TRAINING = 0
  PROCESSING = 1

  def __init__(self):
    self.state = State.TRAINING
    self.training_data = []
    self.processing_data = []
    self.num_frames = 0

  def process_frame(self, packet):
    self.num_frames = self.num_frames + 1
    # print("Processing frame %d\n" % self.num_frames)
    # TODO: processing stuff here
    pass

  def set_state(self, state):
    if state == State.PROCESSING:
      print("State changed to PROCESSING")
    else:
      print("State changed to TRAINING")
    self.state = state


def wait_for_user_input(state):
  while True:
    wait_read(sys.stdin.fileno())
    ln = sys.stdin.readline()
    if ln.find('P') != -1:
      state.set_state(State.PROCESSING)
    elif ln.find('T') != -1:
      state.set_state(State.TRAINING)


def main():
  state = State()
  headset = emotiv.Emotiv()

  gevent.spawn(headset.setup)
  gevent.sleep(0)
  gevent.spawn(wait_for_user_input, state)
  try:
    while True:
      packet = headset.dequeue()
      state.process_frame(packet)
      gevent.sleep(0)
  except KeyboardInterrupt:
    headset.close()
  finally:
    headset.close()


if __name__ == "__main__":
  main()
