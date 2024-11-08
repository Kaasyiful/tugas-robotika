#!/usr/bin/python3


import random
import math
import matplotlib.pyplot as plt
import yaml

class PRM:
    def __init__(self, num_nodes, num_neighbors, x_limits, y_limits, start, goal):
        self.num_nodes = num_nodes
        self.num_neighbors = num_neighbors
        self.x_limits = x_limits
        self.y_limits = y_limits
        self.start = start
        self.goal = goal
        self.nodes = []
        self.edges = []

    def distance(self, point1, point2):
        return math.sqrt((point1[0] - point2[0])**2 + (point1[1] - point2[1])**2)

    def sample_nodes(self):
        self.nodes.append(self.start)  # Tambahkan node awal
        self.nodes.append(self.goal)  # Tambahkan node tujuan
        for _ in range(self.num_nodes):
            x = random.uniform(*self.x_limits)
            y = random.uniform(*self.y_limits)
            self.nodes.append((x, y))
        print(f"Sampled {len(self.nodes)} nodes.")

    def build_roadmap(self):
        # Hubungkan setiap node dengan tetangga terdekatnya
        for i, node in enumerate(self.nodes):
            # Hitung jarak ke semua node lainnya
            distances = [(self.distance(node, self.nodes[j]), j) for j in range(len(self.nodes)) if j != i]
            distances.sort()  # Urutkan jarak
            for _, neighbor_idx in distances[:self.num_neighbors]:
                neighbor = self.nodes[neighbor_idx]
                self.edges.append((i, neighbor_idx))  # Tambahkan edge
        print("Roadmap constructed.")

    def find_path(self):
        # Algoritma pencarian jalur terpendek sederhana dengan DFS
        visited = set()
        path = []
        if self.dfs(0, 1, visited, path):
            path_coords = [self.nodes[i] for i in path]
            return path_coords
        else:
            print("No path found.")
            return None

    def dfs(self, current, target, visited, path):
        visited.add(current)
        path.append(current)
        if current == target:
            return True
        for edge in self.edges:
            if edge[0] == current and edge[1] not in visited:
                if self.dfs(edge[1], target, visited, path):
                    return True
            elif edge[1] == current and edge[0] not in visited:
                if self.dfs(edge[0], target, visited, path):
                    return True
        path.pop()
        return False

    def visualize(self, path=None):
        plt.figure()
        # Gambar edges
        for edge in self.edges:
            node1, node2 = self.nodes[edge[0]], self.nodes[edge[1]]
            plt.plot([node1[0], node2[0]], [node1[1], node2[1]], 'gray', lw=0.5)

        # Gambar nodes
        plt.scatter(*zip(*self.nodes), color='blue', s=10)
        plt.scatter(*self.start, color='green', s=100, label="Start")
        plt.scatter(*self.goal, color='red', s=100, label="Goal")

        # Gambar jalur jika ditemukan
        if path:
            plt.plot(*zip(*path), color='orange', linewidth=2, label="Path")

        plt.legend()
        plt.show()

def main(config_path):
    with open(config_path, 'r') as file:
        params = yaml.safe_load(file)

    prm = PRM(
        num_nodes=params['num_nodes'],
        num_neighbors=params['num_neighbors'],
        x_limits=params['x_limits'],
        y_limits=params['y_limits'],
        start=tuple(params['start']),
        goal=tuple(params['goal'])
    )

    prm.sample_nodes()
    prm.build_roadmap()
    path = prm.find_path()
    prm.visualize(path)

if __name__ == "__main__":
    import sys
    main(sys.argv[1] if len(sys.argv) > 1 else 'params.yaml')
