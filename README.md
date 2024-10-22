## Rust Cross-Platform Sample App

## Steps to Build and Run
### Android, IOS, Web (WASM), Server
- Check https://fdimarh.medium.com/creating-rust-module-for-multiplatform-application-b16725296d56

### Python 

1. Change directoy to `hello` and build core module.

    ```
    cd hello
    cargo build
    ```

2. Change directoy to `helloPython`

    ```
    cd ../helloPython
    ```

3. Create a new virtual environment in ./venv directory with:

    ```
    python -m venv ./venv
    ```

4. Activate the virtual environment:

    ```
    source ./venv/bin/activate
    ```

5. Install `maturin`

    ```
    pip install maturin
    ```

6.  At this point, we can use Maturin to build and test the project:  

    ```
    maturin build
    maturin develop
    ```
#### Running `python` Module

7. Change directory to python directory. Execute `python main.py`.

    ```
    cd ../python
    python main.py
    ```

#### Running `pythonPoetry` Module

7. Exit virtual environment

8. Install `poetry`

    ```
    curl -sSL https://install.python-poetry.org | python -
    poetry --version
    ```

9. Change directory to `pythonPoetry`

    ```
    cd ../pythonPoetry
    ```

10. Install the project's dependencies as specified in the poetry.lock file.

    ```
    poetry install
    ```
11. Run PythonPoetry project

    ```
    poetry run pythonPoetry
    ```