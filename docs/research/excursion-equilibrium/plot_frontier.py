"""Plot the sharp equilibrium frontier at the first tail of the 3,2,1 ring."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def main():
    m = 23 / 36
    a = np.linspace(np.sqrt(m), 0.848, 1000)
    r = a**3 + m * (a**2 - a)
    frontier = (2*a**4 + a**3 + m*(1+a-a**2)) / ((1-a)*(1+a)**3)
    previous = np.maximum.reduce([
        np.full_like(r, m/(1-m)),
        2*r/(1-r),
        ((1+m)**2/(1-r)-1)/2,
    ])

    fig, ax = plt.subplots(figsize=(7.2, 4.1), layout="constrained")
    ax.fill_between(r, frontier, 2.45, color="#e3f1ed")
    ax.fill_between(r, previous, frontier, color="#efcfa2")
    ax.plot(r, frontier, color="#126758", linewidth=2.4, label="Exact minimum at equilibrium")
    ax.plot(r, previous, color="#784c29", linewidth=1.8, linestyle="--",
            label="Minimum allowed by the earlier tests")
    ax.scatter([11/24], [2], s=55, zorder=4, color="#242424", edgecolors="white")
    ax.annotate("Driven ring  (11/24, 2)", xy=(11/24, 2), xytext=(25, -30),
                textcoords="offset points", ha="left", fontsize=9,
                arrowprops={"arrowstyle": "-", "color": "#333333"})
    ax.set(xlim=(0.414, 0.502), ylim=(1.75, 2.33), xlabel="Fraction with at least two hidden jumps  r",
           ylabel="Mean hidden jumps per excursion  n",
           title="Equilibrium threshold at a fixed first tail  m = 23/36")
    ax.text(0.474, 2.28, "Equilibrium feasible", fontsize=9, color="#104f47",
            ha="center")
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="y", alpha=0.18)
    ax.legend(loc="upper left", frameon=False, fontsize=8.5)
    fig.savefig(Path(__file__).with_name("frontier.png"), dpi=180)
    plt.close(fig)


if __name__ == "__main__":
    main()
