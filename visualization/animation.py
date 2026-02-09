import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

# 1. Load data
df = pd.read_csv('animation_data.csv')
num_pends = df['id'].max() + 1
res = int(np.sqrt(num_pends))  # Grid resolution
frames = df['frame'].unique()

# 2. Angle-to-Color Mapping using turbo colormap
def angles_to_color(theta1, theta2):
    """
    Map angles to colors using the turbo colormap.
    Creates a 2D mapping so different (theta1, theta2) combinations
    span the full color range.
    """
    import matplotlib.cm as cm

    # Normalize angles to [0, 1] range
    h1 = (theta1 % (2 * np.pi)) / (2 * np.pi)
    h2 = (theta2 % (2 * np.pi)) / (2 * np.pi)

    # Create a 2D hash-like mapping that spreads angle combinations
    # across the full [0,1] range for better color distribution
    # Use a combination that creates variation in both directions
    combined = (h1 * 0.5 + h2 * 0.5 +
                0.3 * np.sin(2 * np.pi * h1) * np.cos(2 * np.pi * h2)) % 1.0

    # Apply turbo colormap
    turbo = cm.get_cmap('turbo')
    rgb = turbo(combined)

    # Return RGB only (drop alpha channel)
    return rgb[..., :3]

# 3. Setup Figure (matching visual.py style)
fig, ax = plt.subplots(figsize=(10, 8), facecolor='white')  # Match visual.py figure size and background
ax.set_aspect('equal')
ax.axis('on')  # Turn on axis to show labels like visual.py

# Initial frame
initial = df[df['frame'] == 0].sort_values('id')
t1_0 = initial['theta1'].values.reshape(res, res)
t2_0 = initial['theta2'].values.reshape(res, res)

# Create image with extent matching angle space (in radians like visual.py)
img = ax.imshow(angles_to_color(t1_0, t2_0),
                extent=[0, 2*np.pi, 0, 2*np.pi],
                origin='lower',
                interpolation='gaussian',
                aspect='auto',
                cmap='turbo')  # Use turbo colormap

# Add labels and title (matching visual.py)
plt.xlabel(r'$\theta_1$ (rad)')
plt.ylabel(r'$\theta_2$ (rad)')
ax.set_title(f'Hamiltonian Phase Space Animation ({res}x{res})', fontsize=12)

# Frame counter text (keep this)
frame_text = ax.text(0.02, 0.98, '', transform=ax.transAxes,
                     ha='left', va='top', color='black', fontsize=10,
                     bbox=dict(boxstyle='round', facecolor='white', alpha=0.7))

# 4. Update Function
def update(f):
    # Get data for current frame
    data = df[df['frame'] == f].sort_values('id')

    # Reshape current angles into grid
    t1 = data['theta1'].values.reshape(res, res)
    t2 = data['theta2'].values.reshape(res, res)

    # Update image with new colors based on current angles
    img.set_data(angles_to_color(t1, t2))

    # Update frame counter
    frame_text.set_text(f'Frame: {f}/{len(frames)-1}')

    return [img, frame_text]

# Create animation
ani = FuncAnimation(fig, update, frames=frames, interval=33, blit=True, repeat=True)

plt.tight_layout()  # Match visual.py
plt.show()