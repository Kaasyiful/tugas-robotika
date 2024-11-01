
import heapq
import math

def heuristic(a, b):
    # Menghitung jarak Euclidean sebagai heuristic
    return math.hypot(a[0] - b[0], a[1] - b[1])

def a_star(grid, start, goal):
    rows, cols = len(grid), len(grid[0])
    queue = [(0, start)]
    costs = {start: 0}
    came_from = {start: None}
    
    while queue:
        current_cost, current = heapq.heappop(queue)
        
        if current == goal:
            # Bangun jalur kembali
            path = []
            while current:
                path.append(current)
                current = came_from[current]
            path.reverse()
            print("Jalur ditemukan: ", path)
            #print("total biaya: ", sum(costs.values()))
            return path
        
        for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, 1), (1, -1), (-1, -1), (1, 1)]:
            neighbor = (current[0] + dx, current[1] + dy)
            
            if 0 <= neighbor[0] < rows and 0 <= neighbor[1] < cols and grid[neighbor[0]][neighbor[1]] == 0:
                if (dx == 0 or dy == 0):
                    ds = 1
                else:
                    ds = math.sqrt(2)
                
                new_cost = costs[current] + ds
                
                if neighbor not in costs or new_cost < costs[neighbor]:
                    costs[neighbor] = new_cost
                    priority = new_cost + heuristic(goal, neighbor)
                    heapq.heappush(queue, (priority, neighbor))
                    came_from[neighbor] = current

                    print("jalur yang dijelajahi:", neighbor, "\n")
                    #print("Jarak", neighbor, " dari", current, "= ", ds)
                    #print("Jarak yang ditempuh dari titk mulai", start, "= ", sum(costs.values()), "\n")
    
    print("Jalur tidak ditemukan")
    return None

# Contoh grid: 0 = jalan bebas, 1 = rintangan
grid = [
    [0, 1, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 0, 1, 0]
]

# Panggil algoritma
a_star(grid, (0, 0), (4, 4))
