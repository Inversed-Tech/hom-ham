
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
    loop_parallelize=True
)

SIZE = 1000

@fhe.compiler({"bits": "encrypted"})
def ham(bits):
    total_sum = 0
    for i in range(SIZE):
        total_sum += (bits[i, 0] ^ bits[i, 1])
    return total_sum

data = np.random.randint(0, 2, (SIZE, 2))

print()

print(f"Compilation started @ {time.strftime('%H:%M:%S', time.localtime())}")
start = time.time()
inputset = [np.random.randint(0, 2, (SIZE, 2)) for _ in range(100)]
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
end = time.time()
print(f"(took {end - start:.3f} seconds)")
