use std::collections::BinaryHeap; // Mengimpor struktur data BinaryHeap
use std::cmp::{Ordering, Reverse}; // Mengimpor tipe data Ordering dan Reverse
use std::thread; // Mengimpor modul thread
use std::time::Duration; // Mengimpor tipe data Duration

// Struktur data untuk merepresentasikan tugas robot
#[derive(Eq, PartialEq)]
struct Task {
    priority: u32,           // Prioritas tugas
    task_type: TaskType,    // Jenis tugas
    target: (i32, i32),        // Koordinat target
}

// Enum untuk jenis tugas robot
#[derive(Eq, PartialEq)]
enum TaskType {
    Move,
    Pickup,
    Deliver,
    Charge,
}

// Implementasi untuk Task
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority) // Urutan ascending (nilai terkecil dulu)
    }
}

// Implementasi untuk Task
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Mengembalikan hasil dari fungsi cmp
    }
}

// Struktur untuk robot
struct Robot {
    position: (i32, i32),                      // Posisi robot dalam koordinat 2D
    tasks: BinaryHeap<Reverse<Task>>,        // Antrian tugas robot
    current_task: Option<Task>,            // Tugas yang sedang dikerjakan
}

// Implementasi untuk Robot
impl Robot {
    fn new() -> Self {
        Robot {
            position: (0, 0), // Posisi awal robot
            tasks: BinaryHeap::new(), // Inisialisasi antrian tugas kosong
            current_task: None, // Tidak ada tugas yang sedang dikerjakan
        }
    }

    // Menambahkan tugas ke antrian
    fn add_task(&mut self, task: Task) {
        println!("Adding task with priority {}", task.priority);
        self.tasks.push(Reverse(task)); // Menambahkan tugas ke antrian
    }

    // Memproses tugas berikutnya
    fn process_next_task(&mut self) {
        // Mengambil tugas dengan prioritas tertinggi
        if let Some(Reverse(task)) = self.tasks.pop() {
            println!("Processing task with priority {}", task.priority);
            // Menyimpan tugas yang sedang dikerjakan
            match task.task_type {
                TaskType::Move => self.move_to(task.target),
                TaskType::Pickup => self.pickup_at(task.target),
                TaskType::Deliver => self.deliver_to(task.target),
                TaskType::Charge => self.charge_at(task.target),
            }
        }
    }

    // Menggerakkan robot ke posisi target
    fn move_to(&mut self, target: (i32, i32)) {
        println!("Moving to position {:?}", target);
        self.position = target;
    }

    // Mengambil objek pada posisi target
    fn pickup_at(&mut self, location: (i32, i32)) {
        println!("Picking up object at {:?}", location);
        self.move_to(location);
    }

    // Mengantarkan objek ke posisi tujuan
    fn deliver_to(&mut self, destination: (i32, i32)) {
        println!("Delivering to {:?}", destination);
        self.move_to(destination);
    }

    // Mengisi daya baterai di stasiun pengisian
    fn charge_at(&mut self, station: (i32, i32)) {
        println!("Charging at station {:?}", station);
        self.move_to(station);
    }
}

fn main() {
    let mut robot = Robot::new(); // Membuat robot baru

    // Menambahkan beberapa tugas ke antrian robot
    robot.add_task(Task {
        priority: 3, // Prioritas tugas
        task_type: TaskType::Move, // Jenis tugas
        target: (5, 5), // Koordinat target
    });
    
    robot.add_task(Task {
        priority: 1,
        task_type: TaskType::Charge,
        target: (0, 0),
    });

    robot.add_task(Task {
        priority: 2,
        task_type: TaskType::Pickup,
        target: (3, 4),
    });

    // Memproses tugas-tugas yang ada dalam antrian
    while !robot.tasks.is_empty() {
        robot.process_next_task();
        thread::sleep(Duration::from_secs(1)); // Menunggu 1 detik
    }

    println!("All tasks completed!"); // Menampilkan pesan semua tugas selesai
}