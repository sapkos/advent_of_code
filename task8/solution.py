import re

with open('input.txt') as f:
    lines = f.read()
    data = list(map(int, re.findall('\d+', lines)))


def data_generator(data=data):
    for d in data:
        yield d

class Node:
    def __init__(self, _children=[], _metadata=[]):
        self.children = list(_children)
        self.metadata = list(_metadata)

def create_nodes(data):
    no_child = next(data)
    no_meta = next(data)
    if no_child == 0:
        meta = [next(data) for _ in range(no_meta)]
        return [Node(_metadata=meta)]
    else:
        return [Node(_children=[create_nodes(data) for _ in range(no_child)], 
                    _metadata=[next(data) for _ in range(no_meta)])]


dg = data_generator()

tree = create_nodes(dg)

def add_metadata(tree):
    _sum = sum(tree[0].metadata)
    for children in tree[0].children:
        _sum += add_metadata(children)
    return _sum

print(add_metadata(tree))


def add_indexed_metadata(tree):
    _sum = 0
    for nd in tree:
        if not nd.children:
            _sum += sum(nd.metadata)
        else:
            for m in nd.metadata:
                if len(nd.children) >= m:
                    _sum += add_indexed_metadata(nd.children[m-1])
    return _sum

print(add_indexed_metadata(tree))
