import math
import numpy as np
from numpy.polynomial import Polynomial as P
import matplotlib.pyplot as plt

# Benchmark data
# Here are the sizes of MSM we tried
xs =    [16    , 32    , 64    , 128  , 256    , 512   , 1024  , 2048    , 4096   , 8192   , 16384   , 32768   , 65536   , 131072   , 262144  , 524288, 1048576, 2097152, 4194304]
# The time it took in milliseconds for the MSM in G1
ys_G1 = [1.2, 1.6, 1.0, 3.0, 5.2, 4.1, 7.0, 12.9 , 23.5, 39.3, 72.0,  139.4, 265.8, 489.96, 836.4, 1000.54, 3000.55 , 5000.75 , 11000.9]
# The time it took in milliseconds for the MSM in G2
ys_G2 = [3.4, 3.6, 5.2, 8.0, 9.9, 11.0,18.5, 34.5, 61.9, 106 , 193.6, 374.0, 662.0, 1000.28,    2000.32,   4000.19, 9000.76 , 15000.7 , 31000.7]

# Heuristic on how many ms it takes to do a G1 MSM of size `n`
def msm_G1_ms(n):
    complexity = 254 + (254*n)/math.log(254*n,2)
    nano = 315 * complexity
    return (nano / 1000) / 1000

# Heuristic on how many ms it takes to do a G2 MSM of size `n`
def msm_G2_ms(n):
    complexity = 254 + (254*n)/math.log(254*n,2)
    nano = 880 * complexity
    return (nano / 1000) / 1000

plt.plot(xs, ys_G1, label="G1 bench")
plt.plot(xs, ys_G2, label="G1 bench")

heuristic_G1_ys = []
heuristic_G2_ys = []
for x in xs:
    heuristic_G1_ys.append(msm_G1_ms(x))
    heuristic_G2_ys.append(msm_G2_ms(x))


plt.plot(xs, heuristic_G1_ys, label="G1 heuristic")
plt.plot(xs, heuristic_G2_ys, label="G2 heuristic")

plt.legend()
plt.show()
