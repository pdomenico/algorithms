graph = {
    "s": ["a", "b"],
    "a": ["s", "c"],
    "b": ["s", "c", "d"],
    "c": ["a", "b", "d", "e"],
    "d": ["b", "c", "e"],
    "e": ["c", "d"],
}


def shortest_paths(start: str, graph: dict[str, list[str]]) -> dict[str, int]:
    paths: dict[str, int] = {}
    queue: list[str] = [start]

    while len(queue) > 0:
        print(f"Queue is: {queue}")
        elem = queue.pop()
        if elem == start:
            paths[elem] = 0

        subnodes = graph.get(elem)
        if subnodes == None:
            continue
        else:
            for subnode in subnodes:
                if not subnode in paths:
                    queue.append(subnode)
                    paths[subnode] = paths[elem] + 1

    return paths


print(shortest_paths("s", graph))
