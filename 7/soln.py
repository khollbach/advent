#!/usr/bin/python3

def parse():
    edges = []
    for line in open('input'):
        words = line.split()

        # Reversed, for easier top. sort
        node1 = words[7]
        node2 = words[1]

        edges.append((node1, node2))

    nodes = set()
    for node1, node2 in edges:
        nodes.add(node1)
        nodes.add(node2)

    # Adjacency lists
    graph = {}
    for node in sorted(nodes):
        if node not in graph:
            graph[node] = set()
    for node1, node2 in edges:
        if node2 not in graph[node1]:
            graph[node1].add(node2)

    return graph

def readyNodes(graph, ignore=[]):
    nodes = []
    for node, edges in graph.items():
        if not edges:
            nodes.append(node)
    return [node for node in nodes if node not in ignore]

def removeNode(graph, node):
    assert not graph[node]  # Sanity check: node should be empty

    graph.pop(node)
    for otherEdges in graph.values():
        if node in otherEdges:
            otherEdges.remove(node)

def topSort(graph):
    if not graph:
        return []

    for node in readyNodes(graph):
        removeNode(graph, node)
        return [node] + topSort(graph)

    assert False  # Cyclic :(

def part1():
    graph = parse()

    order = topSort(graph)

    print(''.join(order))

def timeToComplete(node):
    return 61 + ord(node) - ord('A')

def part2():
    graph = parse()

    readyWorkers = 5
    workingNodes = {}  # node -> finishTime

    currTime = 0
    while graph:
        readyNodes_ = readyNodes(graph, ignore=workingNodes.keys())
        if readyWorkers > 0 and readyNodes_:
            readyWorkers -= 1
            node = readyNodes_.pop(0)
            workingNodes[node] = currTime + timeToComplete(node)
        else:
            assert readyWorkers < 5  # Otherwise graph is cyclic!

            # Advance time until one or more nodes complete.
            currTime = min(workingNodes.values())
            for node in list(workingNodes.keys()):
                if workingNodes[node] == currTime:
                    removeNode(graph, node)
                    workingNodes.pop(node)
                    readyWorkers += 1

    print(currTime)

#part1()
part2()
