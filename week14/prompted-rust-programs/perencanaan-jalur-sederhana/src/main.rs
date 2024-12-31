// 1. Perencanaan Jalur Sederhana

/*
    Prompt: 
    Tolong buatkan kode Rust untuk merencanakan jalur robot dari titik awal (0, 0) ke tujuan (4, 4) dalam sebuah matriks 2D. Rintangan ditandai dengan angka 1, dan jalan bebas dengan angka 0. Jalur harus menghindari rintangan.
*/

use std::collections::{BinaryHeap, HashMap}; // Mengimpor struktur data bertipe BinaryHeap dan HashMap
use std::cmp::Ordering;                      // Mengimpor tipe data Ordering

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {                            // Struct untuk posisi
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug)]
struct Node {                                // Struct untuk node dalam A*
    pos: Position,
    f_score: i32,                           // Total biaya dari start ke goal 
    g_score: i32,                           // Biaya dari start ke node saat ini
}

impl Ord for Node {                         // Penentuan urutan node berdasarkan f_score
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)    // Urutan descending (nilai terbesar dulu)
    }
}

impl PartialOrd for Node {                  // Penentuan urutan node berdasarkan f_score
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(pos: &Position, goal: &Position) -> i32 {
    (pos.x - goal.x).abs() + (pos.y - goal.y).abs() // Menghitung jarak Manhattan antara dua posisi sebagai heuristik
}

fn get_neighbors(pos: &Position, grid: &Vec<Vec<i32>>) -> Vec<Position> {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Daftar arah tetangga yang mungkin
    // Inisialisasi vektor untuk menyimpan posisi tetangga yang valid
    let mut neighbors = Vec::new();
    // Iterasi untuk setiap arah tetangga
    for (dx, dy) in dirs.iter() {
        let new_x = pos.x + dx; // Menghitung koordinat x tetangga
        let new_y = pos.y + dy; // Menghitung koordinat y tetangga
        
        if new_x >= 0 && new_x < grid[0].len() as i32 &&
           new_y >= 0 && new_y < grid.len() as i32 &&
           grid[new_y as usize][new_x as usize] == 0 {
            neighbors.push(Position { x: new_x, y: new_y });
        } // Menambahkan posisi tetangga jika valid
    }
    neighbors // Mengembalikan vektor posisi tetangga
}

// Fungsi untuk mencari jalur dari start ke goal
fn find_path(grid: &Vec<Vec<i32>>, start: Position, goal: Position) -> Option<Vec<Position>> {
    let mut open_set = BinaryHeap::new(); // Binary heap untuk menyimpan node yang akan dieksplorasi
    let mut came_from = HashMap::new();  // HashMap untuk menyimpan node tetangga yang terbaik
    let mut g_scores = HashMap::new();  // HashMap untuk menyimpan biaya dari start ke node saat ini
    
    g_scores.insert(start, 0); // Inisialisasi biaya dari start ke start adalah 0
    open_set.push(Node {
        pos: start,
        f_score: manhattan_distance(&start, &goal),
        g_score: 0,
    }); // Menambahkan start ke open_set dengan f_score dan g_score yang sesuai

    while let Some(current) = open_set.pop() {
        // Jika node saat ini adalah goal, maka jalur ditemukan
        if current.pos == goal {
            let mut path = vec![goal]; // Inisialisasi vektor untuk menyimpan jalur
            let mut current_pos = goal; // Inisialisasi posisi saat ini sebagai goal
            // Iterasi untuk mendapatkan jalur dari goal ke start
            while let Some(&pos) = came_from.get(&current_pos) {
                path.push(pos);  // Menambahkan posisi saat ini ke jalur
                current_pos = pos; // Update posisi saat ini
            }
            path.reverse(); // Membalikkan jalur agar dari start ke goal
            return Some(path); // Mengembalikan jalur
        }

        // Iterasi untuk setiap tetangga dari node saat ini
        for neighbor in get_neighbors(&current.pos, grid) {
            let tentative_g_score = g_scores[&current.pos] + 1; // Menghitung biaya dari start ke tetangga

            // Jika tetangga belum dieksplorasi atau biaya dari start ke tetangga lebih kecil
            if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor] {
                came_from.insert(neighbor, current.pos); // Menyimpan node saat ini sebagai tetangga terbaik
                g_scores.insert(neighbor, tentative_g_score); // Menyimpan biaya dari start ke tetangga
                open_set.push(Node {
                    pos: neighbor,
                    f_score: tentative_g_score + manhattan_distance(&neighbor, &goal),
                    g_score: tentative_g_score,
                }); // Menambahkan tetangga ke open_set dengan f_score dan g_score yang sesuai
            }
        }
    }
    None // Mengembalikan None jika jalur tidak ditemukan
}

fn main() {
    // Inisialisasi grid sebagai matriks 2D
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 0, 1, 0, 0],
    ]; // Rintangan ditandai dengan angka 1, dan jalan bebas dengan angka 0

    // Inisialisasi posisi start dan goal
    let start = Position { x: 0, y: 0 };
    let goal = Position { x: 4, y: 4 };

    // Mencari jalur dari start ke goal
    if let Some(path) = find_path(&grid, start, goal) {
        println!("Path found: {:?}", path); // Print jalur yang ditemukan
        
        // Print the grid with path
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let pos = Position { x: x as i32, y: y as i32 }; // Inisialisasi posisi saat ini
                if path.contains(&pos) {
                    print!("* "); // Print '*' jika posisi saat ini ada di jalur
                } else {
                    print!("{} ", grid[y][x]); // Print angka di grid jika posisi saat ini bukan jalur
                }
            }
            println!(); // Pindah ke baris baru
        }
    } else {
        println!("No path found!"); // Print jika jalur tidak ditemukan
    }
}