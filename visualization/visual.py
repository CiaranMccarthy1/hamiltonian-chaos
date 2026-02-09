import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.colors import LogNorm

df = pd.read_csv('chaos_atlas.csv')
res = int(np.sqrt(len(df)))
lle_grid = df['lambda'].values.reshape(res, res)

plt.figure(figsize=(10, 8), facecolor='white')
img = plt.imshow(lle_grid,
                 extent=[0, 2*np.pi, 0, 2*np.pi],
                 origin='lower',
                 cmap='turbo',
                 norm=LogNorm(vmin=0.01, vmax=lle_grid.max()))

plt.colorbar(img, label='Lyapunov Exponent (λ)')
plt.xlabel(r'$\theta_1$ (rad)')
plt.ylabel(r'$\theta_2$ (rad)')
plt.title(f'Hamiltonian Phase Space Atlas ({res}x{res})')
plt.tight_layout()
plt.show()