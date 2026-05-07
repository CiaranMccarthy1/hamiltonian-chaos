import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def calculate_fractal_dimension(df, threshold=None):
    res = int(np.sqrt(len(df)))
    L = df['lambda'].values.reshape(res, res)
    L_clean = np.clip(L, 1e-9, 10.0)
    L_log = np.log10(L_clean)

    if threshold is None:
        threshold = np.mean(L_log)

    binary_map = (L_log > threshold).astype(int)

    diff_x = np.diff(binary_map, axis=1, append=binary_map[:, -1:]) != 0
    diff_y = np.diff(binary_map, axis=0, append=binary_map[-1:, :]) != 0
    boundary = (diff_x | diff_y).astype(int)

    if np.sum(boundary) == 0:
        return None

    def count_boxes(img, size):
        n_boxes = res // size
        clipped_res = n_boxes * size
        clipped_img = img[:clipped_res, :clipped_res]
        reduced = clipped_img.reshape(n_boxes, size, n_boxes, size)
        return np.sum(np.any(reduced, axis=(1, 3)))

    sizes = [2, 3, 4, 5, 6, 8, 10, 12, 15, 20]
    raw_counts = [count_boxes(boundary, s) for s in sizes]

    valid_sizes = []
    valid_counts = []
    for s, c in zip(sizes, raw_counts):
        if c > 0:
            valid_sizes.append(s)
            valid_counts.append(c)

    x = np.log(1.0 / np.array(valid_sizes))
    y = np.log(valid_counts)
    slope, intercept = np.polyfit(x, y, 1)

    plt.figure(figsize=(12, 5))

    plt.subplot(1, 2, 1)
    plt.imshow(boundary, cmap='hot', origin='lower')
    plt.title(f"Fractal Boundary (D ≈ {slope:.3f})")
    plt.axis('off')

    plt.subplot(1, 2, 2)
    plt.plot(x, y, 'ro')
    plt.plot(x, slope*x + intercept, 'b-')
    plt.xlabel('log(1/epsilon)')
    plt.ylabel('log(N)')
    plt.title('Regression Analysis')
    plt.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.show()

    return slope

if __name__ == "__main__":
    data = pd.read_csv('chaos_atlas.csv')
    dim = calculate_fractal_dimension(data)
    print(f"Fractal Dimension: {dim}")