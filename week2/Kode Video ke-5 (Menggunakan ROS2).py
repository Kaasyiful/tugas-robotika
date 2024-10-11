#!/usr/bin/env python3
import rclpy
from rclpy.node import Node

class TestNode(Node):
    def __init__(self):
        super().__init__('test_node')
        self.timer =self.create_timer(0.1, self.loop)

    def loop(self):
        self.get_logger().info('Hello')

if __name__ == '__main__':
    rclpy.init()
    test_node = TestNode()
    test_node.get_logger().info('Test node has been started')
    rclpy.spin(test_node)
    test_node.destroy_node()
    rclpy.shutdown()