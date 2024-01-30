import pandas as pd
import numpy as np
import os
from sklearn.preprocessing import MinMaxScaler
from sklearn.model_selection import train_test_split
from keras.models import Sequential
from keras.layers import LSTM, Dense, Dropout


data = pd.read_csv(os.path.join(os.getcwd(), "data-collect", "data", "merged_parsed_data.csv")) 
# print(data.info())
# data = data.iloc[:, 1:18]
y = data["spy_close"].fillna(method="ffill") # SPY Close, our target variable, index # 11, Column 12
y = y.values.reshape(-1, 1)

scaler = MinMaxScaler(feature_range=(0, 1))
y = scaler.fit_transform(y)

n_lookback = 12
n_forecast = 3

X = [] #array of input sequences
Y = [] 

for i in range(n_lookback, len(y) - n_forecast+1):
    X.append(y[i - n_lookback: i]) #appending an input sequence to X
    Y.append(y[i : i + n_forecast]) #output sequence, Y[i] represents forecast based on X[i]

X = np.array(X)
Y = np.array(Y)

model = Sequential([
    LSTM(50, input_shape=(n_lookback, 1), return_sequences=True),
    LSTM(50, return_sequences=False),
    #Dropout(0.2),
    Dense(n_forecast)
])

model.compile(optimizer='adam', loss='mean_squared_error')
model.fit(X, Y, epochs=100, batch_size=32, verbose=0)

#forecast generation
X_ = y[-n_lookback:]
X_ = X_.reshape(1, n_lookback, 1)

Y_ = model.predict(X_).reshape(-1, 1)
Y_ = scaler.inverse_transform(Y_)
print(Y_)
# predicted_value_matrix = np.zeros(shape=(len(Y_), 17))
# predicted_value_matrix[:, 11] = Y_.flatten()
# predicted_value = scaler.inverse_transform(predicted_value_matrix)[:, 11]

# print(predicted_value)

#organize results in df
df_past = data[["spy_close"]].reset_index()
df_past.rename(columns={"index": "Date", "spy_close": "Actual"}, inplace=True)
df_past["Date"] = pd.to_datetime(df_past["Date"])
df_past["Forecast"] = np.nan
df_past["Forecast"].iloc[-1] = df_past["Actual"].iloc[-1]

df_future = pd.DataFrame(columns=["Date", "Actual", "Forecast"])
df_future["Date"] = pd.date_range(start=df_past["Date"].iloc[-1] + pd.Timedelta(minutes=15), periods=n_forecast)
df_future["Forecast"] = Y_.flatten()
df_future["Actual"] = np.nan

results = df_past.append(df_future).set_index("Date")

#results plot
results.plot(title="SPY")
