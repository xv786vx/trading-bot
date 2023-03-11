from dotenv import load_dotenv
# import time
import requests
import os
import json

baseURL = "https://api.twelvedata.com/"
endpoint = "time_series"
url = baseURL + endpoint

load_dotenv()

API_KEYS = [os.getenv("API_KEY_1"), os.getenv("API_KEY_2")]


def getSingleHistoricalData(regionalSymbol: str, api_key_index: int):

    params = {
        "symbol": regionalSymbol,
        "interval": "1min",
        "outputsize": 5000,
        "apikey": API_KEYS[api_key_index]
    }

    response = requests.get(url, params=params)
    data = json.loads(response.json()["values"])
    print(json.dumps(data, indent=4, sort_keys=True))


def getRegionalHistoricalData(interval: int, api_key_index: int):
    symbol = "?symbol=NYSE,NASDAQ,NSE,TSE,SSE,HKEX,LSE,SPY"
    url = "https://api.twelvedata.com/time_series" + symbol + \
        "&interval=" + interval + "&outputsize=5000&apikey=" + \
        API_KEYS[api_key_index]
    response = requests.get(url)
    data = json.loads(response.text)
    print(json.dumps(data, indent=4, sort_keys=True))


getRegionalHistoricalData("15min", 0)
# getSingleHistoricalData("SPY")
