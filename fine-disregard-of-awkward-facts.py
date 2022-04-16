from gpiozero import LED
from time import sleep
from gpiozero import DistanceSensor

led = LED(18)
sensor = DistanceSensor(echo=22, trigger=27)

while True:
    led.on()
    sleep(1)
    led.off()
    print('Distance: ', sensor.distance * 100)
    sleep(1)