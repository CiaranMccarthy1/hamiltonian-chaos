import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from matplotlib import cm

# ==========================================
# 1. PARAMETERS (Match your Rust code)
# ==========================================
M1, M2 = 1.0, 1.0
L1, L2 = 1.0, 1.0
G = 9.81

def get_potential_energy(t1, t2):
    """Calculates Potential Energy V where 0 is stable equilibrium."""
    return (M1 + M2) * G * L1 * (1 - np.cos(t1)) + M2 * G * L2 * (1 - np.cos(t1 + t2))

def get_chaos_score(t1, t2, lambda_val):
    """
    Example Thesis Formula: Predicts chaos based on Energy and Coupling.
    S = (Energy Ratio) * (Mass-Length Coupling)
    """
    V = get_potential_energy(t1, t2)
    E_flip = 2 * M2 * G * L2  # Energy needed to flip the second arm
    coupling = (M2 / (M1 + M2)) * (L2 / L1)

    score = (V / E_flip) * coupling
    return score

# ==========================================
# 2. PART 1: 3D ENERGY-STABILITY LANDSCAPE
# ==========================================
def plot_3d_atlas(filename='chaos_atlas.csv'):
    df = pd.read_csv('chaos_atlas.csv')
    res = int(np.sqrt(len(df)))
    L = df['lambda'].values.reshape(res, res)
    T1 = df['theta1'].values.reshape(res, res)
    T2 = df['theta2'].values.reshape(res, res)

    # 2. LOG TRANSFORMATION
    # We add a tiny offset (1e-6) so we don't take the log of zero
    # This 'evens out' the black/purple areas
    L_log = np.log10(L + 1e-6)

    # 3. 3D PLOT
    fig = plt.figure(figsize=(12, 8))
    ax = fig.add_subplot(111, projection='3d')

    # Potential Energy for Z-axis
    V = (1.0 + 1.0) * 9.81 * 1.0 * (1 - np.cos(T1)) + 1.0 * 9.81 * 1.0 * (1 - np.cos(T1 + T2))

    # Normalize the Log values for the colormap
    norm = plt.Normalize(L_log.min(), L_log.max())
    colors = cm.magma(norm(L_log))

    surf = ax.plot_surface(T1, T2, V, facecolors=colors,
                           shade=False, rcount=res, ccount=res, antialiased=True)

    # 4. Colorbar with Log Labels
    m = cm.ScalarMappable(cmap=cm.magma, norm=norm)
    cbar = fig.colorbar(m, ax=ax, shrink=0.5, label='$\log_{10}(lambda)$')

    plt.show()

# ==========================================
# EXECUTION
# ==========================================
if __name__ == "__main__":
    # 1. First, show the scientific landscape
    plot_3d_atlas('a.csv')

