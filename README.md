# morphrs-py

Experimental [morph-rs](https://github.com/kribrum-os/morph-rs) bindings for Python.

Demonstration: [test.ipynb](test.ipynb)

Current workflow:

1. Install rust, python (3.8 shoud be supported, tested on 3.10)
2. Create a virtual environment: `python -m venv .venv`
3. Activate the virtual environment: `source .venv/bin/activate`
4. Install dependencies: `pip install -r dev_requirements.txt`
5. Build and install morphrs-py: `maturin develop`
6. Download the dictionary from morph-rs releases:

```bash
wget --quiet https://github.com/kribrum-os/morph-rs/releases/download/v0.2.0/dict.zip && unzip dict.zip -d dict
```

Now you can use `morphrs_py` from python:

```python
from morphrs_py import MorphAnalyzer

analyzer = MorphAnalyzer.open("dict/")
steel = morph_analyzer.parse_word("стали")
print(steel)
```

Also, you can run the `test.ipynb` using jupyter notebook:

```bash
jupyter notebook test.ipynb
```

## License

The code of this project is licensed under the [Apache License 2.0](https://github.com/insolor/morphrs-py/blob/main/LICENSE).

The code of [morph-rs](https://github.com/kribrum-os/morph-rs/) is licensed under the [Kribrum-NC](https://github.com/kribrum-os/morph-rs/blob/main/license.md) (Apache License 2.0 based) license.
