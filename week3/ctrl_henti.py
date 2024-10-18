"""ctrl_henti controller."""

# You may need to import some classes of the controller module. Ex:
#  from controller import Robot, Motor, DistanceSensor
from controller import Robot, DistanceSensor

# create the Robot instance.
robot = Robot()

# get the time step of the current world.
timestep = int(robot.getBasicTimeStep())

proximity_sensors = []
sensor_names = ['ps0', 'ps1', 'ps2', 'ps3', 'ps4', 'ps5', 'ps6', 'ps7']
for name in sensor_names:
    sensor:DistanceSensor = robot.getDevice(name)
    sensor.enable(timestep)
    proximity_sensors.append(sensor)

speed = 6.28

left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

left_motor.setPosition(float('inf'))
right_motor.setPosition(float('inf'))

left_motor.setVelocity(speed)
right_motor.setVelocity(speed)

# Main loop:
# - perform simulation steps until Webots is stopping the controller
while robot.step(timestep) != -1:
    # Cek nilai sensor proximity
    frontright_distance = proximity_sensors[0].getValue()
    frontleft_distance = proximity_sensors[7].getValue()
    print("Front Right Distance: " , frontright_distance)
    print("Front Left Distance: " , frontleft_distance)
    
    if frontright_distance > 100 or frontleft_distance > 100:
        left_motor.setVelocity(0)
        right_motor.setVelocity(0)
    else:
        left_motor.setVelocity(speed)
        right_motor.setVelocity(speed)
    pass

# Enter here exit cleanup code.
