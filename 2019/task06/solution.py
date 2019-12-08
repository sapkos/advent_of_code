def get_sum_of_edges(planets, starting_planet, starting_weight):
    try:
        planets[starting_planet]
    except KeyError:
        return 0
    return (len(planets[starting_planet].orbit) * starting_weight
            + sum(get_sum_of_edges(planets, p, starting_weight + 1)
                  for p in planets[starting_planet].orbit)
            )

with open('input.txt') as f:
    pairs = [p.strip().split(')') for p in f.readlines()]

class Planet:
    def __init__(self, name):
        self.name = name
        self.orbit = []
        self.orbits = None
        self.see_santa = False
        self.santa_distance = -1

    def add_orbit(self, planet):
        self.orbit.append(planet)

    def __repr__(self):
        return f"Planet: {self.name}, orbit: {self.orbit}, is on orbit of {self.orbits}"

planets = {}

for planet, orbit in pairs:
    if planet not in planets:
        planets[planet] = Planet(planet)
    if orbit not in planets:
        planets[orbit] = Planet(orbit)

    planets[planet].add_orbit(orbit)
    planets[orbit].orbits = planet


print(get_sum_of_edges(planets, 'COM', 1))

x = 'SAN'
distance = 1
while x != 'COM':
    x_planet = planets[x]
    parent_planet = planets[x_planet.orbits]
    parent_planet.see_santa = True
    parent_planet.santa_distance = distance
    x = parent_planet.name
    distance += 1

x = planets['YOU']
distance = 0
while not x.see_santa:
    x = planets[x.orbits]
    distance += 1

print(distance + x.santa_distance - 2)
