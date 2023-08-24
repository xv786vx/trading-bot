import pandas as pd
import numpy as np
import os
from sklearn.preprocessing import MinMaxScaler
from sklearn.model_selection import train_test_split
from keras.models import Sequential
from keras.layers import LSTM, Dense, Dropout

scaler = MinMaxScaler(feature_range=(0, 1))
data = pd.read_csv(os.path.join(os.getcwd(), "data-collect", "data", "merged_filtered_data.csv"), header=None)  # Replace with your CSV file
# data = data.iloc[:, 1:18]
data = scaler.fit_transform(np.array(data.iloc[:, 12]).reshape(-1, 1))


y = data # SPY Close, our target variable, index # 11, Column 12
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
    LSTM(32, dropout=0.2, input_shape=(n_lookback, 1), return_sequences=True),
    LSTM(16, return_sequences=False),
    #Dropout(0.2),
    Dense(n_forecast, kernel_regularizer='l2')
])
model.compile(optimizer='adam', loss='mean_squared_error')
model.fit(X, Y, epochs=80, batch_size=16, verbose=0)

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
#df_past = data.iloc[:, 10].reset_index()

