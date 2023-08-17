import numpy as np

sample = np.arange(25)
print(sample)

norm_sample = []
stand_sample = []

norm_stand_sample = []

for num in sample:
    norm_sample.append((num - sample.min()) / (sample.max() - sample.min()))
    stand_sample.append((num - sample.mean()) / sample.std())

    norm_stand_sample.append(
        (((num - sample.min()) / (sample.max() - sample.min())) - sample.mean()) / sample.std()
    )

print(norm_sample)
print(stand_sample)
print(norm_stand_sample)