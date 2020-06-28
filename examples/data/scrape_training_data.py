import requests
import math
import pandas as pd

import time
import progressbar

from math import ceil

SYMBOL = 'ETH'
BEGIN_TIMESTAMP = 1483228800 # Jan 1, 2017, 00:00
N_POINTS = 365*24*3
OUTPUT_FILE = 'ethusd_hourly.csv'
CRYPTOCOMPARE_API_LIMIT = 2000
DELAY = 5

def get_endpoint(timestamp):
    return 'https://min-api.cryptocompare.com/data/histohour?fsym=%s&tsym=USD&limit=2000&aggregate=1&toTs=%d'%(SYMBOL, timestamp)

# We have to scrape in multiple steps, because CryptoCompare API has a
# limit of 2000 data points
target_timestamps = [BEGIN_TIMESTAMP+i*60*60*CRYPTOCOMPARE_API_LIMIT \
                     for i in range(math.ceil(N_POINTS/CRYPTOCOMPARE_API_LIMIT))]

bar = progressbar.ProgressBar(max_value=len(target_timestamps))
# Get the data into a dict to make collecting and sorting easier
result_dict = {}
for n, target_timestamp in enumerate(target_timestamps):

    response = requests.get(get_endpoint(target_timestamp))
    data = response.json()['Data']

    for point in data:
        result_dict[point['time']] = point['close']

    bar.update(n)
    time.sleep(DELAY)

# Get points from the dict
points = list(result_dict.items())
# Sort them w.r.to timestamp
points = sorted(points, key=lambda p: p[0])
# Clip extra data
points = points[-N_POINTS:]

# Write the data to the output file
ofile = open(OUTPUT_FILE, 'w')
ofile.write('#timestamp,%s_price\n'%SYMBOL)

for point in points:
    ofile.write('%d,%e\n'%(point[0], point[1]))

