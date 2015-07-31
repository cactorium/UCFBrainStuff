import brainstuff2 as bs2

def quality_check(quals):
    return len([q for q in quals if q > 10]) > 8

def extract_quality(packet):
    return [q for (v, q) in packet.values]

def run_quality_check_loop(func, packet_gen, *args):
    packet = packet_gen.next()
    good = quality_check(extract_quality(packet))
    still_run = True
    print("Waiting for sufficiently good signals...")
    while not good:
        packet = packet_gen.next()
        good = quality_check(extract_quality(packet))
    print("Signals good!")
    while good and still_run:
        still_run = func(packet, *args)
        packet = packet_gen.next()
        good = quality_check(extract_quality(packet))

def main():
    packets = bs2.packets()

if __name__ == "__main__":
    main()
