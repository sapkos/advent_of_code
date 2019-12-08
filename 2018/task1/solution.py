def frequencies(file_name='input.txt'):
    with open(file_name) as f:
        for line in f:
            yield int(line)

def endless_frequencies(file_name='input.txt'):
    while(True):
        with open(file_name) as f:
            for line in f:
                yield int(line)


print(sum(frequencies()))

_sum = 0

frequencies_dict = {}

for freq in endless_frequencies():
    _sum += freq
    frequencies_dict[_sum] = frequencies_dict.get(_sum, 0) + 1
    if frequencies_dict[_sum] == 2:
        break

print(_sum)
