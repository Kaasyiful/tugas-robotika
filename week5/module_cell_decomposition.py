import numpy as np
import math

def cell_decomposition(grid):
    rows, cols = grid.shape
    cells = []
    for row in range(rows):
        for col in range(cols):
            if grid[row, col] == 0:  # 0 menandakan cell bebas
                cells.append((row, col))
    return cells

def heuristic(a, b):
    # Menggunakan jarak Euclidean sebagai heuristic
    return math.hypot(a[0] - b[0], a[1] - b[1])

def find_path_in_cells(cells, start, end):
    path = []
    visited = set()  # Set untuk menyimpan cell yang telah dikunjungi
    current = start
    while current != end:
        path.append(current)
        visited.add(current)

        # Mencari tetangga yang memungkinkan
        next_steps = [
            (current[0] + dx, current[1] + dy) for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, 1), (1, -1), (-1, -1), (1, 1)]
        ]
        next_steps = [step for step in next_steps if step in cells and step not in visited]

        # Periksa jika tidak ada langkah berikutnya
        while len(next_steps) < 1:
            # Mundur jika terjebak
            if len(path) > 1:
                print("kembali dari", path[-1])
                path.pop()  # Kembali ke cell sebelumnya
                current = path[-1]

                next_steps = [
                    (current[0] + dx, current[1] + dy) for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, 1), (1, -1), (-1, -1), (1, 1)]
                    ]
                next_steps = [step for step in next_steps if step in cells and step not in visited]
                continue

            else:
                return None  # Tidak ada solusi

        # Urutkan langkah berikutnya berdasarkan heuristic jarak ke tujuan
        next_steps.sort(key=lambda s: heuristic(s, end))
        print("urutan tetangga berdasarkan heuristik:", next_steps)

        # Pilih langkah terbaik berikutnya
        current = next_steps[0]
        print("tetangga yang terpilih:", next_steps[0])

    path.append(end)
    return path

# Contoh grid 5x5
grid = np.array([
    [0, 1, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 0, 1, 0],
    [0, 0, 0, 1, 0]
])
start, end = (0, 0), (4, 4)
cells = cell_decomposition(grid)
path = find_path_in_cells(cells, start, end)
print("Path:", path)
