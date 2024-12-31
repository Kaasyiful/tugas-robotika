use std::collections::VecDeque; // Mengimpor struktur data VecDeque

// Struktur untuk merepresentasikan posisi dalam koordinat 2D
#[derive(Clone, Copy, PartialEq, Debug)]
struct Point {
    x: i32, // Koordinat x
    y: i32, // Koordinat y
}

// Struktur untuk robot
struct Robot {
    position: Point, // Posisi robot
    goal: Point,    // Tujuan robot
}

impl Robot {
    // Membuat instance robot baru
    fn new(start: Point, goal: Point) -> Self {
        Robot {
            position: start,
            goal,
        }
    }

    // Mengecek apakah posisi valid (dalam peta dan tidak ada obstacle)
    fn is_valid_move(&self, point: Point, map: &Vec<Vec<bool>>, size: i32) -> bool {
        point.x >= 0 
            && point.x < size 
            && point.y >= 0 
            && point.y < size 
            && !map[point.x as usize][point.y as usize]
    }

    // Mencari jalur menggunakan BFS
    fn find_path(&self, map: &Vec<Vec<bool>>, size: i32) -> Option<Vec<Point>> {
        let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // kanan, bawah, kiri, atas
        let mut visited = vec![vec![false; size as usize]; size as usize]; // Matriks untuk menyimpan status kunjungan
        let mut queue = VecDeque::new(); // Antrian untuk BFS
        let mut prev = vec![vec![None; size as usize]; size as usize]; // Matriks untuk menyimpan posisi sebelumnya
        
        queue.push_back(self.position); // Menambahkan posisi awal ke antrian
        visited[self.position.x as usize][self.position.y as usize] = true; // Menandai posisi awal sebagai sudah dikunjungi

        // Melakukan pencarian jalur
        while let Some(current) = queue.pop_front() {
            if current == self.goal {
                // Merekonstruksi jalur
                let mut path = Vec::new(); 
                let mut curr = current; // Menggunakan posisi tujuan sebagai titik akhir
                while curr != self.position {
                    path.push(curr);
                    curr = prev[curr.x as usize][curr.y as usize].unwrap();
                }
                path.push(self.position); // Menambahkan posisi awal ke jalur
                path.reverse(); // Membalik jalur agar dimulai dari posisi awal
                return Some(path); // Mengembalikan jalur yang ditemukan
            }

            // Mencoba semua gerakan yang mungkin
            for (dx, dy) in moves.iter() {
                let next = Point {
                    x: current.x + dx,
                    y: current.y + dy,
                };

                // Jika gerakan valid dan belum dikunjungi, tambahkan ke antrian
                if self.is_valid_move(next, map, size) && !visited[next.x as usize][next.y as usize] {
                    queue.push_back(next);
                    visited[next.x as usize][next.y as usize] = true;
                    prev[next.x as usize][next.y as usize] = Some(current);
                }
            }
        }
        None // Tidak ada jalur yang ditemukan jika antrian kosong
    }
}

fn main() {
    let size = 10;
    // Membuat peta 10x10 (false = kosong, true = obstacle)
    let mut map = vec![vec![false; size]; size];
    
    // Menambahkan beberapa rintangan
    map[0][3] = true;
    map[2][3] = true;
    map[3][2] = true;
    map[6][6] = true;
    map[6][7] = true;
    map[7][6] = true;

    // Membuat robot dengan posisi awal dan tujuan
    let robot = Robot::new(
        Point { x: 0, y: 0 },     // Start
        Point { x: 9, y: 9 }      // Goal
    );

    // Mencari dan menampilkan jalur
    if let Some(path) = robot.find_path(&map, size as i32) {
        println!("Jalur yang ditemukan:"); // Menampilkan pesan jalur ditemukan
        for (i, point) in path.iter().enumerate() {
            println!("Langkah {}: ({}, {})", i + 1, point.x, point.y); // Menampilkan langkah dan posisi
        }
    } else {
        println!("Tidak ada jalur yang ditemukan!");
    }
}