class Node:
    name: str
    outgoing_edges: list
    label: int
    visited: bool

    def __init__(self, name: str):
        self.name = name
        self.outgoing_edges = []
        self.label = 0
        self.visited = False

    def __str__(self):
        return f"Node name: {self.name}, label: {self.label}"


def topological_order(graph: list[Node]) -> int:
    for node in graph:
        if node.visited:
            return 0
        

    
    
# Create graph and nodes
s = Node("s")
v = Node("v")
w = Node("w")
t = Node("t")
graph = [s, v, w, t]
s.outgoing_edges = [v, w]
v.outgoing_edges = [t]
w.outgoing_edges = [t]

global n
n = len(graph)
topological_order(graph)

for node in graph:
    print(node)
