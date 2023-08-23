import pandas as pd
import numpy as np
import os
from sklearn.preprocessing import MinMaxScaler
from sklearn.model_selection import train_test_split
from keras.models import Sequential
from keras.layers import LSTM, Dense, Dropout

# Step 1: Load data
data = pd.read_csv(os.path.join(os.getcwd(), "data-collect", "data", "merged_filtered_data.csv"), header=None)  # Replace with your CSV file
data = data.iloc[:, 1:18]

# Step 2: Feature Selection
selected_features = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]  # Column indices

# Step 3: Sequence Length and Target
sequence_length = 12  # Choose an appropriate sequence length
target_column = 11  # Close price of SPY
target_shift = -1  # To predict the next time step

# Step 4: Data Scaling
scaler = MinMaxScaler()
scaled_data = scaler.fit_transform(data)

# Step 5: Creating Input Sequences
input_sequences = []
target_values = []
print(len(scaled_data))
for i in range(len(scaled_data) - sequence_length):
    input_sequences.append(scaled_data[i : i + sequence_length])
    target_values.append(scaled_data[i + sequence_length + target_shift, target_column])

input_sequences = np.array(input_sequences)
print(scaled_data[5, 11])
target_values = np.array(target_values)

# Step 6: Splitting Data
x_train, x_temp, y_train, y_temp = train_test_split(input_sequences, target_values, test_size=0.2, shuffle=False)
x_val, x_test, y_val, y_test = train_test_split(x_temp, y_temp, test_size=0.5, shuffle=False)

# Step 7: LSTM Model
model = Sequential([
    LSTM(32, dropout=0.2, input_shape=(sequence_length, len(selected_features)), return_sequences=True),
    LSTM(16, return_sequences=False),
    Dropout(0.2),
    Dense(1, kernel_regularizer='l2')
])


# Step 8: Model Training
model.compile(optimizer='adam', loss='mean_squared_error')
model.fit(x_train, y_train, validation_data=(x_val, y_val), epochs=50, batch_size=32)

print("Test Loss:", model.evaluate(x_test, y_test))

# Step 9: Predictions
# future_input_sequence = scaled_data[-sequence_length:, selected_features]  # Replace with appropriate future data
# future_input_sequence = np.expand_dims(future_input_sequence, axis=0)
# predicted_scaled_value = model.predict(future_input_sequence)
# predicted_scaled_value = predicted_scaled_value.reshape(len(predicted_scaled_value[0]))

# # Step 10: Inverse Scaling
# predicted_value_matrix = np.zeros(shape=(len(predicted_scaled_value), 17))
# predicted_value_matrix[:, 10] = predicted_scaled_value
# predicted_prices = scaler.inverse_transform(predicted_value_matrix)[:, 10]

# # print(f"Predicted future Close price of SPY: {predicted_value}")
# for i, prices in enumerate(predicted_prices):
#     print(f"Predicted prices at t+{i+1}: {prices}")