use std::collections::VecDeque;  // Mengimpor struktur data VecDeque
use std::thread;                   // Mengimpor modul thread
use std::time::Duration;          // Mengimpor tipe data Duration

// Struktur untuk menyimpan status robot
struct RobotState {
    position: (f64, f64),      // Posisi robot dalam koordinat 2D
    direction: f64,            // Arah hadap robot dalam derajat
    target: (f64, f64),       // Tujuan robot dalam koordinat 2D
    obstacles: Vec<(f64, f64)> // Vektor untuk menyimpan posisi rintangan
}

// Enum untuk event yang dapat diterima oleh robot
enum RobotEvent {
    NewObstacle((f64, f64)),  // Rintangan baru terdeteksi
    TargetChanged((f64, f64)), // Tujuan robot berubah
    SensorUpdate              // Pembaruan sensor
}

// Implementasi untuk RobotState
impl RobotState {
    fn new() -> Self {
        RobotState {
            position: (0.0, 0.0),
            direction: 0.0,
            target: (10.0, 10.0),
            obstacles: Vec::new()
        }
    }

    // Menghandle event yang diterima oleh robot
    fn handle_event(&mut self, event: RobotEvent) {
        // Memproses event berdasarkan jenisnya
        match event {
            // Jika terdeteksi rintangan baru
            RobotEvent::NewObstacle(pos) => {
                println!("New obstacle detected at {:?}", pos);
                self.obstacles.push(pos);
                self.adjust_path();
            },
            // Jika tujuan berubah
            RobotEvent::TargetChanged(pos) => {
                println!("Target changed to {:?}", pos);
                self.target = pos;
                self.adjust_path();
            },
            // Jika terjadi pembaruan sensor
            RobotEvent::SensorUpdate => {
                self.update_position();
            }
        }
    }

    // Menyesuaikan jalur robot untuk menghindari rintangan
    fn adjust_path(&mut self) {
        // Cek apakah ada rintangan terdekat
        if let Some(nearest) = self.find_nearest_obstacle() {
            let dx = self.position.0 - nearest.0; // Menghitung selisih koordinat x
            let dy = self.position.1 - nearest.1; // Menghitung selisih koordinat y
            self.direction = dy.atan2(dx).to_degrees(); // Mengubah sudut ke derajat
        }
    }

    // Memperbarui posisi robot berdasarkan arah hadap
    fn update_position(&mut self) {
        // Menghitung perubahan posisi berdasarkan arah hadap
        let rad = self.direction.to_radians();
        self.position.0 += rad.cos() * 0.1; // Mengubah posisi x
        self.position.1 += rad.sin() * 0.1; // Mengubah posisi y
        println!("Position updated to: {:?}", self.position); // Menampilkan posisi baru
    }

    // Mencari rintangan terdekat dari posisi robot
    fn find_nearest_obstacle(&self) -> Option<(f64, f64)> {
        // Menggunakan iter() untuk mengakses elemen vektor
        self.obstacles.iter()
            .min_by(|a, b| {
                let dist_a = distance(self.position, **a);
                let dist_b = distance(self.position, **b);
                dist_a.partial_cmp(&dist_b).unwrap() // Membandingkan jarak antara dua rintangan
            })
            .copied() // Mengambil nilai terkecil
    }
}

// Fungsi untuk menghitung jarak antara dua titik
fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt() // Menghitung jarak Euclidean
}

fn main() {
    // Inisialisasi robot dan antrean event
    let mut robot = RobotState::new();
    let mut event_queue: VecDeque<RobotEvent> = VecDeque::new();

    // Loop utama untuk simulasi robot
    loop {
        // Menunggu sebentar untuk simulasi waktu
        thread::sleep(Duration::from_millis(1000));
        
        // Menambahkan rintangan baru secara acak
        if rand::random::<f64>() < 0.1 {
            event_queue.push_back(RobotEvent::NewObstacle((
                rand::random::<f64>() * 20.0 - 10.0,
                rand::random::<f64>() * 20.0 - 10.0
            )));
        }

        // Menambahkan kemungkinan perubahan target secara random
        if rand::random::<f64>() < 0.05 {
            event_queue.push_back(RobotEvent::TargetChanged((
                rand::random::<f64>() * 20.0 - 10.0,
                rand::random::<f64>() * 20.0 - 10.0
            )));
        }

        // Mengubah tujuan robot secara acak
        event_queue.push_back(RobotEvent::SensorUpdate);

        // Memproses event yang ada dalam antrean
        while let Some(event) = event_queue.pop_front() {
            robot.handle_event(event);
        }

        // Menghentikan simulasi jika robot mencapai tujuan
        if distance(robot.position, robot.target) < 0.5 {
            println!("Target reached!");
            break;
        }
    }
}