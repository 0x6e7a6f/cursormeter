# Cursormeter
## Introduction
Useless program made in Rust which showing you the distance done by your mouse cursor.

---

## How does it work ?

Basically, It's an equation of a line from two coordinates on a Cartesian plane.

$$\text{distance}(A,B)=\sqrt{(x_{2}-x_{1})^{2}+(y_{2}-y_{1})^{2}}$$

The program retrieves the position of the mouse cursor every 20 milliseconds, so with 2 samples we can perform the operation. In fact, the longer the time between each iteration to retrieve a sample, the more inaccurate the metrics will be.
