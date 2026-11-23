"""Entropy over a single four-state family with a fixed observable clock."""
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def entropy(u):
    v = 1 / (16 * u)
    return 4 * (u + v) * (np.arctanh(2 * u) + np.arctanh(2 * v))


def main():
    u = np.linspace(1/8 + 1e-7, 1/2 - 1e-7, 4000)
    fig, ax = plt.subplots(figsize=(7.0, 3.9), layout="constrained")
    ax.plot(u, entropy(u), color="#165d68", lw=2.4)
    ax.axhline(2*np.log(3), ls="--", lw=1.0, color="#6b757c")
    for bias, label, offset in [(1/4, "Minimum  ·  2 ln 3", (17, -18)),
                                (4/24, "Factory setting (9, 4)", (27, 42))]:
        ax.scatter(bias, entropy(bias), color="#9c4328", s=31, zorder=3)
        ax.annotate(label, (bias, entropy(bias)), xytext=offset,
                    textcoords="offset points", fontsize=9,
                    arrowprops={"arrowstyle": "-", "color": "#656565"})
    ax.set(xlim=(1/8, 1/2), ylim=(1.2, 10),
           xticks=[1/8, 1/6, 1/4, 3/8, 1/2],
           xticklabels=["1/8", "1/6", "1/4", "3/8", "1/2"],
           xlabel="Continuation bias  u", ylabel="Entropy per excursion  (k$_B$)",
           title="One jump-clock law, four fixed occupations")
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="y", alpha=0.16)
    fig.savefig(Path(__file__).with_name("fixed-clock.png"), dpi=180)
    plt.close(fig)


if __name__ == "__main__":
    main()
