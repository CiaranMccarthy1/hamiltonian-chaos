import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

def run_square_animation(filename='animation_data.csv'):
    try:
        df = pd.read_csv(filename)
    except FileNotFoundError:
        print(f"Error: {filename} not found.")
        return

    num_pends = df['id'].max() + 1
    res = int(np.sqrt(num_pends))
    frames = sorted(df['frame'].unique())

    def angle_to_rgb(t1, t2):
        # Maps angles to a unique color.
        # Same angles = Same color.
        r = (np.sin(t1) + 1) / 2
        g = (np.sin(t2) + 1) / 2
        b = (np.cos(t1 - t2) + 1) / 2
        return np.stack([r, g, b], axis=-1)

    fig, ax = plt.subplots(figsize=(8, 8), facecolor='black')
    plt.subplots_adjust(0,0,1,1)

    # Initialize grid
    initial = df[df['frame'] == 0].sort_values('id')
    t1_0 = initial['theta1'].values.reshape(res, res)
    t2_0 = initial['theta2'].values.reshape(res, res)

    img = ax.imshow(angle_to_rgb(t1_0, t2_0), origin='lower',
                    interpolation='nearest', animated=True)
    ax.axis('off')

    def update(f):
        data = df[df['frame'] == f].sort_values('id')
        t1 = data['theta1'].values.reshape(res, res)
        t2 = data['theta2'].values.reshape(res, res)
        img.set_array(angle_to_rgb(t1, t2))
        return [img]

    # interval=33 for ~30 FPS (10 seconds total for 300 frames)
    ani = FuncAnimation(fig, update, frames=frames, interval=33, blit=True)
    plt.show()


if __name__ == "__main__":
    # 1. First, show the scientific landscape
    run_square_animation('animation_data.csv')