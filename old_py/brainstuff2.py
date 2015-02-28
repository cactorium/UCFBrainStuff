# This is an example of popping a packet from the Emotiv class's packet queue
# and printing the gyro x and y values to the console.

from emokit.emotiv import Emotiv
import platform
if platform.system() == "Windows":
    import socket  # Needed to prevent gevent crashing on Windows. (surfly / gevent issue #459)
import gevent

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation

is_running = True
# TODO: is_running is not working as expected. But it DOES work!

def packets():
    global is_running
    ring_buf = np.zeros(x.size)
    headset = Emotiv()

    gevent.spawn(headset.setup)
    gevent.sleep(0)
    pos = 0
    try:
        while is_running:
            packet = headset.dequeue()
            data = {key: (value["value"], value["quality"])
                    for (key, value) in packet.sensors.items()}
            yield data
            gevent.sleep(0)
    except KeyboardInterrupt:
        headset.close()
    finally:
        is_running = False
        headset.close()

def yield_helper():
    gevent.sleep(0)
