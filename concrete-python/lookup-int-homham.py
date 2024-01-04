
import time

import matplotlib.pyplot as plt
import numpy as np
from concrete import fhe

configuration = fhe.Configuration(
    enable_unsafe_features=True,
    use_insecure_key_cache=True,
    insecure_key_cache_location=".keys",

    # To enable displaying progressbar
    show_progress=True,
    # To enable showing tags in the progressbar (does not work in notebooks)
    progress_tag=True,
    # To give a title to the progressbar
    progress_title="Evaluation:",
)

# 2^10, 1 bit
#SIZE = 1024
#MOD = 2

# 2^9, 2 bits
#SIZE = 512
#MOD = 4

# 2^8, 4 bits
#SIZE = 256
#MOD = 16

# 2^7, 8 bits
SIZE = 128
MOD = 128

# 2^6, 16 bits
#SIZE = 64
#MOD = 65536

# 2^5, 32 bits
#SIZE = 32
#MOD = 4294967296



table = fhe.LookupTable(
    [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4,
     1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
     1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
     2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
     1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
     2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
     2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
     3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
     1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
     2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
     3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
     2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
     3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
     3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
     4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8])


@fhe.compiler({"bits": "encrypted"})
def ham(bits):
    total_sum = 0
    for i in range(SIZE):
        c = (bits[i, 0] ^ bits[i, 1])
        total_sum += table[c]
    return total_sum

data = np.random.randint(0, MOD, (SIZE, 2))

print()

print(f"Compilation started @ {time.strftime('%H:%M:%S', time.localtime())}")
start = time.time()
inputset = [np.random.randint(0, MOD, (SIZE, 2)) for _ in range(10)]
circuit = ham.compile(inputset, configuration)
end = time.time()
print(f"(took {end - start:.3f} seconds)")

print()

print(f"Key generation started @ {time.strftime('%H:%M:%S', time.localtime())}")
start = time.time()
circuit.keygen()
end = time.time()
print(f"(took {end - start:.3f} seconds)")

print()

print(f"Evaluation started @ {time.strftime('%H:%M:%S', time.localtime())}")
start = time.time()
eval_data = circuit.encrypt_run_decrypt(data)
print("RESULT:", eval_data)
end = time.time()
print(f"(took {end - start:.3f} seconds)")
