import json
import matplotlib.pyplot as plt
from matplotlib.colors import LogNorm
import numpy as np
import datetime

# 1. Load file
with open('chaos_results.json', 'r') as f:
    data = json.load(f)

# 2. Automatically detect resolution
res = int(len(data)**0.5)

# 3. Extract and reshape the Lyapunov data
lle_values = [d['lyapunov_exponent'] for d in data]
lambda_grid = np.array(lle_values).reshape(res, res)

# 4. Create the plot
plt.figure(figsize=(10, 8))
img = plt.imshow(lambda_grid,
           extent=[0, 2*np.pi, 0, 2*np.pi],
           origin='lower',
           cmap='turbo',
           norm=LogNorm(vmin=0.01, vmax=lambda_grid.max()))

# 5. Labels and styling
plt.colorbar(img, label='Lyapunov Exponent (λ)')
plt.xlabel(r'$\theta_2$ (radians)')
plt.ylabel(r'$\theta_1$ (radians)')
plt.title(f'Double Pendulum Phase Space Atlas ({res}x{res} Resolution)')

plt.tight_layout()
plt.savefig('map.png', dpi=300)
plt.show()