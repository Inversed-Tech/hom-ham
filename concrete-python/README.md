# Using Concrete compiler

This folder contains Python programs based on the Concrete library. A compiler is responsible for generating the underlying circuits.

To run the code, you must install the Concrete library, as described [here](https://docs.zama.ai/concrete/getting-started/installing).
You might also need to install these libraries:
- matplotlib

If you're having trouble installing packages, try using a virtualenv:
```sh
pip install -U pip wheel setuptools virtualenv
python -m venv concrete-venv
source concrete-venv/bin/activate
pip install concrete-python matplotlib
```

After installation, execute `python FILENAME`, where `FILENAME` corresponds to one of the files in this folder.
