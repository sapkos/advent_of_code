import re
from collections import deque, defaultdict

with open('input.txt') as f:
    no_players, last_marble = tuple(map(int, re.findall('\d+', f.readline())))



def play_game(no_players=no_players, last_marble=last_marble):
    player_scores = defaultdict(int)
    marbles = deque([0])

    for marble in range(1, last_marble + 1):
        if marble % 23 == 0:
            marbles.rotate(7)
            player_scores[marble % no_players] += marble + marbles.pop()
            marbles.rotate(-1)
        else:
            marbles.rotate(-1)
            marbles.append(marble)

    return max(player_scores.values())
