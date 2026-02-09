import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import matplotlib.colors as mcolors

# 1. Load data
df = pd.read_csv('animation_data.csv')
num_pends = df['id'].max() + 1
res = int(np.sqrt(num_pends)) # This will be 40
frames = df['frame'].unique()

# 2. Angle-to-Color Mapping
def get_colors(t1, t2):
    # This maps the 2D angle space to a unique RGB color
    # Any two pendulums with the same (t1, t2) will have the same color
    r = (np.sin(t1) + 1) / 2
    g = (np.sin(t2) + 1) / 2
    b = (np.cos(t1 - t2) + 1) / 2
    return np.stack([r, g, b], axis=-1)

# 3. Setup Figure
fig, ax = plt.subplots(figsize=(8, 8), facecolor='black')
# Ensure the plot area is a perfect square
ax.set_aspect('equal')

# Initial Frame
initial = df[df['frame'] == 0].sort_values('id')
t1_0 = initial['theta1'].values.reshape(res, res)
t2_0 = initial['theta2'].values.reshape(res, res)

img = ax.imshow(get_colors(t1_0, t2_0),
                extent=[0, res, 0, res],
                origin='lower',
                interpolation='nearest')

ax.axis('off')
ax.set_title("Square Configuration Space Animation", color='white')

# 4. Update Function
def update(f):
    # Get data for the current frame
    data = df[df['frame'] == f].sort_values('id')

    # Reshape current angles into the 40x40 grid
    t1 = data['theta1'].values.reshape(res, res)
    t2 = data['theta2'].values.reshape(res, res)

    # Update colors
    img.set_data(get_colors(t1, t2))
    return [img]

ani = FuncAnimation(fig, update, frames=frames, interval=33, blit=True, repeat= True)

plt.show()