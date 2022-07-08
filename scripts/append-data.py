# -*- coding: utf-8 -*-
import argparse
import time
from datetime import datetime
import csv
import os

CUR_DIR = os.path.realpath(os.path.dirname(__file__))


def main():

    parser = argparse.ArgumentParser(description="Append timeseries commitment data")
    parser.add_argument("--delay", metavar="delay", type=float, default="0.05")
    args = parser.parse_args()
    delay = args.delay

    data = []
    with open(os.path.join(CUR_DIR, "rts-commitmentment-data.csv")) as f:
        data = csv.reader(f)
        data = [[g for g in row] for row in data]

    for i, generators in enumerate(data):
        if i == 0:
            continue
        with open(os.path.join(CUR_DIR, "../examples/commitment-data.csv"), "a") as f:
            date = datetime.now().strftime("%Y-%m-%dT%H:%M:%S.%f%z")
            generators = ",".join(generators)
            data = "{},{}".format(date, generators)
            print("Writing {} to file".format(data))
            f.write(data)
            f.write("\n")
        print("Sleeping for {} seconds".format(delay))
        time.sleep(delay)


if __name__ == "__main__":
    main()
