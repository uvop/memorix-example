from .schema_generated import Memorix, System
import time

def main() -> None:
    memorix = Memorix()
    memorix.connect()
    memorix.task.pass_ball.empty(System.PYTHON)
    print("Waiting for ball...")
    for ball in memorix.task.pass_ball.dequeue(System.PYTHON):
        bigger_ball = ball + 1
        print("Passing the ball with value {value}".format(value=bigger_ball))
        time.sleep(0.5)
        memorix.task.pass_ball.enqueue(System.DENO, bigger_ball)
