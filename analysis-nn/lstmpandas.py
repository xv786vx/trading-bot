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

df = pd.read_csv(os.path.join(os.getcwd(), "data-collect", "data", "merged_filtered_data.csv"), header=None)

spy_close = df.iloc[:, 10].values
features = df.iloc[:, 1:18].values

timesteps = 100

scaler = MinMaxScaler()
X_scaled = scaler.fit_transform(features)

#reshaping data
num_samples = len(X_scaled) - timesteps + 1

X_reshaped = np.zeros((num_samples, timesteps, X_scaled.shape[1]))
for i in range (num_samples):
    X_reshaped[i] = X_scaled[i:i+timesteps]

y_reshaped = spy_close[timesteps - 1:]

#70 : 15 : 15 split for train, test and validation
X_train, X_temp, y_train, y_temp = train_test_split(X_reshaped, y_reshaped, test_size=0.3, shuffle=False)
X_test, X_val, y_test, y_val = train_test_split(X_temp, y_temp, test_size=0.5, shuffle=False)



#LSTM Model
model = Sequential([
    LSTM(128, return_sequences=True, input_shape=(timesteps, X_train.shape[2])),
    LSTM(32, return_sequences=False),
    Dense(25),
    Dense(1)
])

#compile and train
model.compile(optimizer="adam", loss="mean_squared_error")
history = model.fit(X_train, y_train, epochs=10, batch_size=32, validation_data=(X_val, y_val))

#eval on test set
loss = model.evaluate(X_test, y_test)
print("Test Loss: ", loss)

predictions_scaled = model.predict(X_test)

# workable_matrix = np.zeros(shape=(len(predictions_scaled), 17))
# workable_matrix[:, 10] = predictions_scaled[:, 0]

# #predictions = scaler.inverse_transform(workable_matrix)

# predictions = scaler.inverse_transform(workable_matrix)[:,10]

print(predictions_scaled)
print(y_test)

# y_test_matrix = np.zeros(shape=(len(y_test), 17))
# y_test_matrix[:, 10] = y_test
# y_test = scaler.inverse_transform(y_test_matrix)[:, 10]


plt.figure(figsize=(12, 6))
plt.plot(y_test, label='True')
plt.plot(predictions_scaled, label='Predicted')
plt.title("LSTM Predictions vs True Values")
plt.xlabel("Time")
plt.ylabel("Close Price of SPY")
plt.legend()
plt.show()