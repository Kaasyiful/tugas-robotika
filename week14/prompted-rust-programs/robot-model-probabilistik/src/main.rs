use rand::Rng; // Mengimpor modul rand
use std::f64; // Mengimpor tipe data f64

// Struktur untuk posisi robot
#[derive(Debug, Clone)]
struct Position {
    x: f64, // Koordinat x
    y: f64, // Koordinat y
    uncertainty: f64,  // Tingkat ketidakpastian posisi
}

// Struktur untuk data sensor
#[derive(Debug)]
struct SensorData {
    distance: f64, // Jarak ke target
    angle: f64,   // Sudut terhadap target
    noise: f64,  // Noise/gangguan sensor
}
 // Implementasi untuk struct Position
impl Position {
    // Menghitung jarak ke target
    fn distance_to(&self, target: &Position) -> f64 {
        ((self.x - target.x).powi(2) + (self.y - target.y).powi(2)).sqrt() // Menghitung jarak Euclidean
    }

    // Update posisi dengan mempertimbangkan ketidakpastian
    fn update(&mut self, dx: f64, dy: f64, uncertainty_factor: f64) {
        self.x += dx; // Update koordinat x
        self.y += dy; // Update koordinat y
        self.uncertainty *= uncertainty_factor; // Update ketidakpastian
    }
}

// Fungsi untuk mendapatkan arah gerakan terbaik
fn get_best_path(current: &Position, target: &Position, sensor_data: &SensorData) -> (f64, f64) {
    // Inisialisasi generator angka acak
    let mut rng = rand::thread_rng();
    
    // Menambahkan noise sensor ke perhitungan
    let dx = target.x - current.x + (rng.gen::<f64>() - 0.5) * sensor_data.noise; // Noise pada koordinat x
    let dy = target.y - current.y + (rng.gen::<f64>() - 0.5) * sensor_data.noise; // Noise pada koordinat y
    
    // Normalisasi vektor arah
    let magnitude = (dx.powi(2) + dy.powi(2)).sqrt();
    (dx / magnitude, dy / magnitude)
}

fn main() {
    // Inisialisasi posisi awal robot
    let mut robot_pos = Position {
        x: 0.0,
        y: 0.0,
        uncertainty: 0.1,
    };

    // Posisi target
    let target = Position {
        x: 10.0,
        y: 10.0,
        uncertainty: 0.0,
    };

    // Inisialisasi sensor
    let sensor = SensorData {
        distance: 0.0,
        angle: 0.0,
        noise: 0.2,
    };

    println!("Robot mulai bergerak dari {:?}", robot_pos);
    
    // Simulasi pergerakan robot
    let max_steps = 20; // Jumlah langkah maksimum
    // Loop untuk simulasi pergerakan robot
    for step in 0..max_steps {
        let distance = robot_pos.distance_to(&target); // Hitung jarak ke target
        
        // Cek apakah robot sudah mencapai target
        if distance < 0.5 {
            println!("Target tercapai pada langkah {}!", step);
            break;
        }

        // Dapatkan arah gerakan terbaik
        let (dx, dy) = get_best_path(&robot_pos, &target, &sensor);
        
        // Update posisi dengan ketidakpastian
        robot_pos.update(dx, dy, 1.1); // Ketidakpastian meningkat setiap langkah
        
        println!("Langkah {}: Posisi {:?}, Jarak ke target: {:.2}", 
                step, robot_pos, distance); // Tampilkan posisi dan jarak ke target
    }
}