from dotenv import load_dotenv
# import time
import requests
import os
import json
import pandas as pd

baseURL = "https://api.twelvedata.com/"
endpoint = "time_series"
url = baseURL + endpoint

load_dotenv()

API_KEYS = [os.getenv("API_KEY_1"), os.getenv("API_KEY_2")]
pd.set_option('display.max_rows', None)
pd.set_option('display.max_columns', None)


def getSingleHistoricalData(regionalSymbol: str, api_key_index: int):

    params = {
        "symbol": regionalSymbol,
        "interval": "1min",
        "outputsize": 5000,
        "apikey": API_KEYS[api_key_index]
    }

    response = requests.get(url, params=params).json()
    
    tabledResponse = pd.DataFrame(response['values'])
    print(tabledResponse)

    data = json.loads(response.json()["values"])
    print(json.dumps(data, indent=4, sort_keys=True))


def getRegionalHistoricalData(interval: int, api_key_index: int):
    symbol = "?symbol=NYSE,NASDAQ,NSE,TSE,SSE,HKEX,LSE,SPY"
    url = "https://api.twelvedata.com/time_series" + symbol + \
        "&interval=" + interval + "&outputsize=5000&apikey=" + \
        API_KEYS[api_key_index]
    response = requests.get(url).json()
    
    tabledResponse = pd.DataFrame(response)
    print(tabledResponse)


def getMostActiveSPStocks(api_key_index: int):
    #url = "https://api.twelvedata.com/time_series?symbol=SPX&interval=1min&outputsize=100&sort_by=volume&apikey=" + API_KEYS[api_key_index]
    url = "https://api.twelvedata.com/stocks?exchange=NASDAQ&sort_by=volume&order=desc&outputsize=10&apikey=" + API_KEYS[api_key_index]

    response = requests.get(url)
    data = json.loads(response.text)
    #print(json.dumps(data, indent=4))
    file = open("stock_data.json", "w")
    file.write(json.dumps(data, indent=4))
    file.close()


    #data = response.json()
    
    print(json.dumps(data, indent=4))


getMostActiveSPStocks(1)
#getRegionalHistoricalData("15min", 0)
#getSingleHistoricalData("SPY",0)
