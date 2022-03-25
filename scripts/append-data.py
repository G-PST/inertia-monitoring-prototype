# -*- coding: utf-8 -*-
import argparse
import random
import time
from datetime import datetime


def main():

    parser = argparse.ArgumentParser(description="Append timeseries commitment data")
    parser.add_argument("generators", metavar="ngen", type=int)
    parser.add_argument("--timesteps", metavar="timesteps", type=int, default="1")
    parser.add_argument("--delay", metavar="delay", type=float, default="0.05")
    args = parser.parse_args()

    ngen = args.generators
    timesteps = args.timesteps
    delay = args.delay
    with open("examples/commitment-data.csv", "a") as f:
        for _ in range(0, timesteps):
            date = datetime.now().strftime("%Y-%m-%dT%H:%M:%S.%f%z")
            generators = ",".join([str(random.randint(0, 1)) for _ in range(0, ngen)])
            data = "{},{}".format(date, generators)
            print("Writing {} to file".format(data))
            f.write(data)
            f.write("\n")
            time.sleep(delay)


if __name__ == "__main__":
    main()
