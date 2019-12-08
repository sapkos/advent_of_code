import re
from collections import defaultdict

PLANTS_MATCHER = re.compile('[#.]+')

def file_opener():
    with open('input.txt') as f:
        for line in f:
            if line != '\n':
                yield PLANTS_MATCHER.findall(line[:-1])

class Rule:
    def __init__(self, _input):
        pattern, result = _input
        self._pattern = pattern
        self.result = result
        self.left = pattern[:2]
        self.middle = pattern[2]
        self.right = pattern[3:]

    def __repr__(self):
        return f"{self._pattern} => {self.result}"

class Garden:
    def __init__(self, initial_state):
        self._state = initial_state

    def grow_garden(self, rules):
        self._state = f"..{self._state}.."
        new_state = ['.', '.']
        for no, plant in enumerate(self._state[2:-2]):
            for rule in rules:
                if self.match_rule(no, rule):
                    new_state.append(rule.result)
                    break
            else:
                new_state.append(plant)
        self._state = ''.join(new_state)

    def __repr__(self):
        return self._state

    def match_rule(self, no, rule):
        return (
            (self.middle(no) == rule.middle)
            and (self.left(no) == rule.left)
            and (self.right(no) == rule.right)
        )

    def middle(self, no):
        return self._state[no+2]

    def left(self, no):
        return self._state[no-2:no]

    def right(self, no):
        return self._state[no+2+1:no+2+2+1]

garden = defaultdict(lambda: '.')

fo = file_opener()

current_state = next(fo)[0]

rules = list(Rule(r) for r in fo)

print(current_state)
print(rules)
