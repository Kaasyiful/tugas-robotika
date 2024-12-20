from controller import Robot
import numpy as np

# Parameter Webots
TIME_STEP = 32

# Inisialisasi robot
robot = Robot()

# Sensor roda
left_motor = robot.getDevice("left wheel motor")
right_motor = robot.getDevice("right wheel motor")
left_motor.setPosition(float('inf'))  # Mode kecepatan
right_motor.setPosition(float('inf'))  # Mode kecepatan
left_motor.setVelocity(0.0)
right_motor.setVelocity(0.0)

# Encoder roda
left_encoder = robot.getDevice("left wheel sensor")
right_encoder = robot.getDevice("right wheel sensor")
left_encoder.enable(TIME_STEP)
right_encoder.enable(TIME_STEP)

# Sensor jarak (misalnya untuk pengukuran posisi)
distance_sensor = robot.getDevice("distance sensor")
distance_sensor.enable(TIME_STEP)

# Kalman Filter
def kalman_filter(z, u, x, P):
    # Prediksi langkah
    x_pred = x + u
    P_pred = P + np.eye(len(P)) * 0.1  # Noise proses

    # Koreksi langkah
    K = P_pred / (P_pred + 1)  # Gain Kalman
    x_new = x_pred + K * (z - x_pred)  # Pembaruan posisi
    P_new = (1 - K) * P_pred  # Pembaruan ketidakpastian

    return x_new, P_new

# Variabel untuk posisi dan ketidakpastian
x = np.array([0])  # Posisi awal
P = np.array([[1]])  # Ketidakpastian awal

# Loop utama
while robot.step(TIME_STEP) != -1:
    # Ambil nilai encoder
    left_distance = left_encoder.getValue()
    right_distance = right_encoder.getValue()

    # Estimasi pergerakan robot (input u)
    u = (left_distance + right_distance) / 2.0

    # Ambil pengukuran sensor jarak (z)
    z = distance_sensor.getValue()

    # Terapkan Kalman Filter
    x, P = kalman_filter(z, u, x, P)

    print("Estimasi Posisi Robot: ", x)
