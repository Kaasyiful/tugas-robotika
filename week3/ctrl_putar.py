"""ctrl_putar controller."""

# You may need to import some classes of the controller module. Ex:
#  from controller import Robot, Motor, DistanceSensor
from controller import Robot

# create the Robot instance.
robot = Robot()

# get the time step of the current world.
timestep = int(robot.getBasicTimeStep())
speed = 6.28

# Mendapatkan referensi motor kiri dan kanan
leftMotor = robot.getDevice('left wheel motor')
rightMotor = robot.getDevice('right wheel motor')

# Menyetel posisi motor agar berjalan tanpa batas
leftMotor.setPosition(float('inf'))
rightMotor.setPosition(float('inf'))

# Mengatur kecepatan motor kiri lebih lambat dari kanan
leftMotor.setVelocity(0.5 * speed)  # Roda kiri lebih lambat
rightMotor.setVelocity(speed)       # Roda kanan lebih cepat

# Main loop:
# - perform simulation steps until Webots is stopping the controller
while robot.step(timestep) != -1:
    # Read the sensors:
    # Enter here functions to read sensor data, like:
    #  val = ds.getValue()

    # Process sensor data here.

    # Enter here functions to send actuator commands, like:
    #  motor.setPosition(10.0)
    pass

# Enter here exit cleanup code.
