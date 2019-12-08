from string import ascii_lowercase

with open('input.txt') as f:
    word = f.read()

stosik = []

def is_reacting(a, b):
    return ((a==b.lower()) and (a.upper()==b)) or ((a.lower()==b) and (a==b.upper()))

for char in word:
    if stosik and is_reacting(stosik[-1], char):
        stosik.pop()
    else:
        stosik.append(char)

print(len(stosik)-1)

minimum = len(word)
for x in ascii_lowercase:
    stosik = []
    for char in word.replace(x, '').replace(x.upper(), ''):
        if stosik and is_reacting(stosik[-1], char):
            stosik.pop()
        else:
            stosik.append(char)
    
    minimum = min(len(stosik)-1, minimum)

print(minimum)

