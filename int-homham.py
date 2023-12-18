
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

SIZE = 64
MOD = 256

@fhe.compiler({"i": "encrypted"})
def popcount_16bit(i):
    #assert 0 <= i < 0x10000
    i = i - ((i >> 1) & 0x5555)
    i = (i & 0x3333) + ((i >> 2) & 0x3333)
    i = ((i + (i >> 4)) & 0xF0F) * 0x101
    return (i >> 8) & 0xFF

@fhe.compiler({"bits": "encrypted"})
def ham(bits):
    total_sum = 0
    for i in range(SIZE):
        i = (bits[i, 0] ^ bits[i, 1])
        i = i - ((i >> 1) & 0x55)
        i = (i & 0x33) + ((i >> 2) & 0x33)
        i = ((i + (i >> 4)) & 0xF) * 0x11
        i = i & 0xFF
        total_sum += i
    return total_sum

data = np.random.randint(0, 2, (SIZE, MOD))

print()

print(f"Compilation started @ {time.strftime('%H:%M:%S', time.localtime())}")
start = time.time()
inputset = [np.random.randint(0, 2, (SIZE, MOD)) for _ in range(10)]
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
