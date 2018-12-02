from collections import Counter

cnt_2 = 0
cnt_3 = 0

with open('input.txt') as f:
    for line in f:
        c = Counter(line)
        values = set(map(lambda x: x[1], c.most_common()))
        cnt_2 += 1 if 2 in values else 0
        cnt_3 += 1 if 3 in values else 0

print(cnt_2 * cnt_3)

with open('input.txt') as f:
    lines = f.readlines()
    for line_1 in lines:
        for line_2 in lines:
            dif = 0
            for ch_1, ch_2 in zip(line_1, line_2):
                if ch_1 != ch_2:
                    dif += 1
            if dif == 1:
                ans = [ch_1 for ch_1, ch_2 in zip(line_1, line_2) if ch_1==ch_2]

print(''.join(ans))
