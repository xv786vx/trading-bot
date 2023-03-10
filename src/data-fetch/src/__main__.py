import requests
import time
import os
from dotenv import load_dotenv

baseURL = "https://api.twelvedata.com/"
endpoint = "time_series"
url = baseURL + endpoint

load_dotenv()

API_KEY = os.getenv('API_KEY_1')


def getSingleHistoricalData(regionalSymbol):
    
    params = {
    "symbol": regionalSymbol,
    "interval": "5min",
    "outputsize": 5000,
    "apikey": API_KEY
    }
    response = requests.get(url, params=params)
    data = response.json()["values"]
    print(data)

    
getSingleHistoricalData("AAPL")

