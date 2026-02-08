import json
import matplotlib.pyplot as plt
import numpy as np

with open('chaos_results.json') as f:
    data = json.load(f)

# Reshape into 36x36 grid
lambda_grid = np.zeros((36, 36))
for d in data:
    i = int(d['theta1_initial'] / (2*np.pi) * 36)
    j = int(d['theta2_initial'] / (2*np.pi) * 36)
    lambda_grid[i, j] = d['lyapunov_exponent']

plt.imshow(lambda_grid, cmap='hot', origin='lower')
plt.colorbar(label='Lyapunov Exponent (1/s)')
plt.xlabel('θ₂ (radians)')
plt.ylabel('θ₁ (radians)')
plt.title('Double Pendulum Chaos Map')
plt.show()