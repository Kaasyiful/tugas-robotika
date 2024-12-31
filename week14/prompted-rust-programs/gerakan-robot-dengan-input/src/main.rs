use std::io::{self, Write}; // Mengimpor fungsi write dari modul io

// Struktur untuk menyimpan posisi robot
struct Position {
    x: i32,
    y: i32,
}

// Struktur untuk robot
struct Robot {
    position: Position,
}

impl Robot {
    // Membuat robot baru pada posisi (0,0)
    fn new() -> Self {
        Robot {
            position: Position { x: 0, y: 0 },
        }
    }

    // Menggerakkan robot berdasarkan perintah
    fn move_to(&mut self, direction: &str) {
        match direction.trim().to_lowercase().as_str() {
            "up" => self.position.y += 1,
            "down" => self.position.y -= 1,
            "right" => self.position.x += 1,
            "left" => self.position.x -= 1,
            _ => println!("Perintah tidak valid!"), // Menampilkan pesan jika perintah tidak valid
        }
    }

    // Menampilkan posisi robot
    fn display_position(&self) {
        println!("Posisi robot: ({}, {})", self.position.x, self.position.y);
    }
}

fn main() {
    let mut robot = Robot::new(); // Membuat robot baru
    robot.display_position(); // Menampilkan posisi awal robot

    loop {
        print!("Masukkan perintah (up/down/right/left/quit): ");
        io::stdout().flush().unwrap(); // Memastikan prompt tercetak

        let mut input = String::new(); // Variabel untuk menyimpan input
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let command = input.trim(); // Menghapus karakter spasi dan newline
        
        if command == "quit" {
            println!("Program selesai!"); // Menampilkan pesan program selesai
            break;
        }

        robot.move_to(command); // Memindahkan robot berdasarkan perintah
        robot.display_position(); // Menampilkan posisi robot
    }
}