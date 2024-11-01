import heapq

def dijkstra(graph, start, goal):
    # Struktur untuk menampung biaya dan jalur
    queue = [(0, start, [])]
    visited = set()
    
    while queue:
        (cost, node, path) = heapq.heappop(queue)
        if node in visited:
            continue
        visited.add(node)
        path = path + [node]
        
        # Jika tujuan ditemukan
        if node == goal:
            print("Jalur terpendek:", path)
            print("Total biaya:", cost)
            return path, cost
        
        # Periksa tetangga
        for neighbor, weight in graph[node]:
            if neighbor not in visited:
                print("Jalur saat ini \t:", path)
                heapq.heappush(queue, (cost + weight, neighbor, path))
                print("Jarak", neighbor, " dari", node, " =", cost + weight)
                

    return None, float('inf')

# Contoh graf berbentuk dictionary
graph = {
    'A': [('B', 1), ('C', 3)],
    'B': [('A', 1), ('D', 2), ('E', 4)],
    'C': [('A', 3), ('F', 5)],
    'D': [('B', 2)],
    'E': [('B', 4), ('F', 2)],
    'F': [('C', 5), ('E', 2)]
}

# Panggil algoritma
dijkstra(graph, 'A', 'F')
