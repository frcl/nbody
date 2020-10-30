import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation
# import toml


def update_line(num, data, *cur_lines):
    for i, line in enumerate(cur_lines):
        line.set_data(data[2*i+1:2*i+3, :num])
    return cur_lines


raw_data = np.loadtxt('data.csv', delimiter=',', unpack=True)
n_bodies = (raw_data.shape[0]-1)//2

# config = toml.load('config.toml')
# dt = config['time_step']
animation_secs = 20
# 1 frame = 40ms -> 25fps
# if step = 40ms/dt -> real time
# if n_frames = 25*animation_secs -> fixed time
n_frames = 25*animation_secs
step = raw_data.shape[1]//n_frames
data = raw_data[:, ::step]

fig = plt.figure()

colors = 'byrgm'
lines = [plt.plot([], [], '-', color=c)[0]
         for _, c in zip(range(n_bodies), colors)]

plt.xlim(-5, 5)
plt.ylim(-5, 5)
ax = plt.axes()
ax.set_aspect(1)

line_ani = animation.FuncAnimation(fig, update_line, n_frames, fargs=[data] + lines,
                                   interval=40, blit=True)
line_ani.save('lines.mp4')
