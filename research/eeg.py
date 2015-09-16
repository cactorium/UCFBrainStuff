#!/usr/bin/env python2.7
from emokit import emotiv
import gevent
from gevent.socket import wait_read

import pprint
import numpy
import sys
import math


pp = pprint.PrettyPrinter()

class State:
  TRAINING = 0
  PROCESSING = 1
  N_STIM_CYCLES = 10
  SEQUENCE_SIZE = 128*16
  SequenceIteration = 0
  SequenceNumber = 0

  def __init__(self):
    self.state = State.TRAINING
    self.training_data = []
    self.processing_data = []
    self.num_frames = 0
    self.corr_coeff = []

  def process_frame(self, packet):
    self.num_frames = self.num_frames + 1
    # print("Processing frame %d\n" % self.num_frames)
    # TODO: processing stuff here
    # Training Stage: (State=0)  User has been instructed to fixate on 1 of 4 targets
    if self.state == State.TRAINING:
      # Only 1 Template T(t) is collected by averaging over N cycles.
      if SequenceNumber < SEQUENCE_SIZE:
        self.training_data[SequenceNumber] += packet.sensors['F3']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['FC6']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['P7']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['T8']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['F7']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['F8']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['T7']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['P8']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['AF4']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['F4']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['AF3']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['O2']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['O1']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['FC5']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['X']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['Y']
        SequenceNumber +=1
      elif SequenceIteration < N_STIM_CYCLES:
        SequenceIteration+=1
        SequenceNumber = 0
        self.training_data[SequenceNumber] += packet.sensors['F3']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['FC6']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['P7']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['T8']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['F7']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['F8']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['T7']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['P8']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['AF4']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['F4']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['AF3']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['O2']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['O1']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['FC5']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['X']
        SequenceNumber +=1
        self.training_data[SequenceNumber] += packet.sensors['Y']
        SequenceNumber +=1
 
      else:
        SequenceIteration = 0
        SequenceNumber = 0
        self.training_data /= 10
        self.state = State.PROCESSING
      # Then,the other 4 templates are CALCULATED by shifting T(t) 
      #pp.pprint(packet.sensors['T7']['value'])
      

    elif self.state == State.PROCESSING:
      # Keep latest segment update if possible
      # N = x = 128 
      if SequenceNumber >= SEQUENCE_SIZE:
        SequenceNumber = 0
      pass

      self.processing_data[SequenceNumber] += packet.sensors['F3']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['FC6']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['P7']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['T8']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['F7']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['F8']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['T7']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['P8']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['AF4']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['F4']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['AF3']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['O2']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['O1']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['FC5']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['X']
      SequenceNumber +=1
      self.processing_data[SequenceNumber] += packet.sensors['Y']
      SequenceNumber +=1

      #Now, find correlation coefficients for all 16 signals!
      #dot product of processing & processing arrays / sqrt (training array squared * processing array squared)
      for x in range (0, 15):
        self.corr_coeff[x] = sum([self.training_data[i] * self.processing_data[i] for i in range(0:SEQUENCE_SIZE)]) /  math.sqrt(sum([self.training_data[i] * self.training_data[i] for i in range(0:SEQUENCE_SIZE)])* sum([self.processing_data[i] * self.processing_data[i] for i in range(0:SEQUENCE_SIZE)])) 

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
