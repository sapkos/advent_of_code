import re

cnt = {}
square_overlaps = {}
claim_ids = {}

with open('input.txt') as f:
    pattern = re.compile('\d+')
    for claim in f:
        print(pattern.findall(claim))
        _id, x0, y0, dx, dy = map(int, pattern.findall(claim))
        for i in range(x0, x0+dx):
            for j in range(y0, y0+dy):
                cnt[(j, i)] = cnt.get((j, i), 0) + 1
                if (j, i) in claim_ids:
                    claim_ids[(j, i)].append(_id)
                
                else:
                    claim_ids[(j, i)] = [_id]
                
                for __id in claim_ids[(j, i)]:
                    if __id in square_overlaps:
                        square_overlaps[__id] = square_overlaps[__id].union(set(claim_ids[(j, i)]))
                    else:
                        square_overlaps[__id] = set(claim_ids[(j, i)])
                



print(len([v for v in cnt.values() if v > 1]))
print([v for v in square_overlaps.values() if len(v) == 1])
