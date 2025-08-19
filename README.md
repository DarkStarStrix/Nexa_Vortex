# Nexa_Vortex

[![Build Status](https://github.com/yourusername/Nexa_Vortex/actions/workflows/rust.yml/badge.svg)](https://github.com/yourusername/Nexa_Vortex/actions/workflows/rust.yml)
[![Coverage Status](https://codecov.io/gh/yourusername/Nexa_Vortex/branch/main/graph/badge.svg?token=YOUR_CODECOV_TOKEN)](https://codecov.io/gh/yourusername/Nexa_Vortex)
[![Code Quality](https://api.codiga.io/project/yourusername/Nexa_Vortex/status/svg)](https://app.codiga.io/hub/project/yourusername/Nexa_Vortex)

Nexa_Vortex is a Python library designed to accelerate computations using GPU resources.
It provides a simple interface for leveraging GPU acceleration in your data processing and machine learning workflows.

## Features

- Easy integration with existing Python projects
- GPU-accelerated computation for improved performance
- Flexible API for various use cases
- Compatible with major GPU libraries

## Installation

First, make sure you have Rust installed and configured.
You may need to build the rust package before installing the python package.

You can install Nexa_Vortex via pip:

```bash
pip install nexa-vortex
```

Or, if you are installed from source:

```bash
git clone https://github.com/yourusername/Nexa_Vortex.git
cd Nexa_Vortex
pip install .
```

**Note:** Ensure that you have built the Rust components before installing the Python package. This usually involves running `cargo build --release` in the appropriate directories (e.g., `nexa_vortex/vortex_core` or `rust/vortex_core`).

## Usage

Import Nexa_Vortex in your Python code:

```python
import nexa_vortex

# Example usage
result = nexa_vortex.accelerate(data)
print(result)
```

Refer to the [documentation](docs/) for detailed API usage and examples.

## Packaging & Distribution

To ship Nexa_Vortex as a package:

1. Ensure your `setup.py` and `pyproject.toml` are correctly configured.
2. Build the package:
    ```bash
    python setup.py sdist bdist_wheel
    ```
3. Upload to PyPI:
    ```bash
    pip install twine
    twine upload dist/*
    ```

## Contributing

Contributions are welcome! Please open issues or submit pull requests via GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or support, please contact [allanw.mk@gmail.com](mailto:allanw.mk@gmail.com).
