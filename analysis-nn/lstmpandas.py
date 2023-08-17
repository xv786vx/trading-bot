# https://www.kaggle.com/code/kushal1506/timeseries-s-p500-stockanalysis-arima-lstm#Loading-the-dataset

# importing libraries

import os
import pandas as pd
import numpy as np
from datetime import datetime
import matplotlib.pyplot as plt
import seaborn as sns
from sklearn.preprocessing import MinMaxScaler
from sklearn.model_selection import train_test_split

#All necessary plotly libraries
import plotly as plotly
import plotly.io as plotly
import plotly.graph_objects as go
import plotly.express as px
from plotly.subplots import make_subplots
from plotly.offline import download_plotlyjs, init_notebook_mode, plot, iplot

# stats tools
import statsmodels.api as sm
from statsmodels.tsa.stattools import adfuller
from statsmodels.tsa.seasonal import seasonal_decompose
from statsmodels.tsa.arima.model import ARIMA
from statsmodels.graphics.tsaplots import plot_acf, plot_pacf

# Arima Model
from pmdarima.arima import auto_arima

# metrics
from sklearn.metrics import mean_squared_error, mean_absolute_error
import math

# LSTM 
from tensorflow import keras
from keras.layers import Dense,LSTM,Dropout,Flatten
from keras import Sequential

# print(os.getcwd())
df = pd.read_csv(os.path.join(os.getcwd(), "data-collect", "data", "merged_normalized_data.csv"), header=None)
# print(df.head(5))
print(df.info())

# y = df.iloc[:, 10]
# X = df.drop(df.columns[10], axis=1)

# X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# X_train = float(X_train)
# X_test = float(X_test)
# y_train = float(y_train)
# y_test = float(y_test)

# X_train = X_train.values.reshape((X_train.shape[0], 1, X_train.shape[1]))
# X_test = X_test.values.reshape((X_test.shape[0], 1, X_test.shape[1]))

# num_features = X_train.shape[2]

# model = Sequential()
# model.add(LSTM(50, return_sequences=True, input_shape=(1, num_features)))
# model.add(Dense(1))
# model.compile(optimizer='adam', loss='mean_squared_error')

# model.fit(X_train, y_train, epochs=50, batch_size=32, verbose=0)
# y_pred = model.predict(X_test)
