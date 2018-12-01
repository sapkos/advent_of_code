def frequencies(file_name='input.txt'):
    with open(file_name) as f:
        for line in f:
            yield int(line)

def endless_frequencies(file_name='input.txt'):
    while(True):
        with open(file_name) as f:
            for line in f:
                yield int(line)


sum = 0

for freq in frequencies():
    sum += freq

print(sum)

sum = 0

frequencies_dict = {}

for freq in endless_frequencies():
    sum += freq
    frequencies_dict[sum] = frequencies_dict.get(sum, 0) + 1
    if frequencies_dict[sum] == 2:
        break

print(sum)
