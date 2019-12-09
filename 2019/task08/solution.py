from collections import Counter
from operator import itemgetter

with open('input.txt') as f:
    photo = f.read().strip()

width = 25
height = 6

layer_len = width * height

no_layers = len(photo) // layer_len

layers = {}

for i in range(no_layers):
    layers[i] = photo[i*layer_len : (i+1)*layer_len]

counts = sorted(map(Counter, layers.values()), key=itemgetter('0'))
least_zeros = counts[0]

print(least_zeros['1'] * least_zeros['2'])

def return_first_color(layers_stack):
    for pixel in layers_stack:
        if pixel in ('0', '1'):
            return pixel
    return pixel

final_picture = []

for i in range(layer_len):
    layers_stack = (layer[i] for layer in layers.values())
    final_picture.append(return_first_color(layers_stack))

print(''.join(final_picture))
