"""This script converts the flags.js file from tar1090 into csv.

flags.js contains a definition of the ICAO allocations for each country.
"""
import csv
import re
import sys


# Compile the regular expression
pattern = re.compile(
    r'{ start: (0x[0-9A-F]+), end: (0x[0-9A-F]+), country: "(.+?)", flag_image: "(.+?)" }'
)


def process_file(path: str):
    data = []
    with open(path) as f:
        for line in f:
            match = pattern.search(line)
            if not match:
                continue
            start: int = int(match.group(1)[2:], 16)
            end: int = int(match.group(2)[2:], 16)
            country: str = match.group(3)
            data.append((f"{start:06x}", f"{end:06x}", country))
    # Write the data as csv, with headers.
    with open("countryranges.csv", "w") as f:
        writer = csv.writer(f)
        writer.writerow(["Start", "End", "Country"])
        writer.writerows(data)


if __name__ == "__main__":
    process_file(sys.argv[1])
