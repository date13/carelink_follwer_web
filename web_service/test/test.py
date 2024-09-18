import sched
import time
from datetime import datetime

import numpy as np

a = [1, 2]
b = list(np.array(a))
b.append(3)
print(a, b)

print(datetime.strptime("2023-1-1", "%Y-%m-%d"))


def time_printer():
    now = datetime.now()
    ts = now.strftime('%Y-%m-%d %H:%M:%S')
    print('do func time :', ts)
    loop_monitor()


def loop_monitor():
    s = sched.scheduler(time.time, time.sleep)  # 生成调度器
    s.enter(5, 1, time_printer, ())
    s.run()


if __name__ == "__main__":
    loop_monitor()
    print("after")
