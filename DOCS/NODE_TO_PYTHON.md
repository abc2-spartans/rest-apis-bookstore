# Node.js/npm to Python (with uv) Equivalents

This guide maps common Node.js/npm commands and concepts to their Python equivalents, with a special focus on the [uv](https://github.com/astral-sh/uv) package manager for Python.

---

## Equivalent of `npm init` in Python (with uv)

```sh
uv pip init
```
- `uv pip init` creates a `requirements.txt` in the current directory if it does not exist.
- For full project setup (virtualenv + requirements), use:

```sh
uv venv .venv
```
- Creates a new virtual environment in `.venv`.

---

## Equivalent of `npm run start` in Python

```sh
python main.py
```
- Runs your Python script (entrypoint).
- With uv, you can also use:

```sh
uv run main.py
```
- This runs the script using the environment managed by uv (uses `.venv` if present).

---

## Equivalent of `npm install <package_name>` in Python (with uv)

```sh
uv pip install <package_name>
```
- Installs the package and its dependencies into the active virtual environment.

---

## Equivalent of `package.json` in Python

- Python does not have a direct equivalent, but uses `requirements.txt` for dependencies.
- For modern projects, `pyproject.toml` (PEP 621) is used for metadata and build configuration.

---

## Equivalent of `npm install` in Python (with uv)

```sh
uv pip install -r requirements.txt
```
- Installs all dependencies listed in `requirements.txt`.

---

## Equivalent of `npm uninstall <package_name>` in Python (with uv)

```sh
uv pip uninstall <package_name>
```
- Uninstalls the package from the environment.

---

## Equivalent of `npm run <script>` in Python

- Python does not have a built-in scripts section like `package.json`.
- Use shell scripts (e.g., `dev.sh`) or Makefiles to define project tasks.

---

## Equivalent of `npx <tool>` in Python

- Use `python -m <module>` to run installed tools/modules, e.g.:

```sh
python -m http.server
```

---

## Equivalent of `JSON.stringify()` in Python

```python
import json
json_string = json.dumps(data)
```

---

## Notes
- [uv](https://github.com/astral-sh/uv) is a modern, fast Python package manager and venv manager.
- Use `.venv` for project-local environments (recommended for isolation).
- For advanced workflows, consider `pyproject.toml` with tools like Poetry or Hatch, but `uv` is fast and simple for most needs.

---

## Equivalent of dotenv (`process.env`) in Python

Install:
```sh
uv pip install python-dotenv
```
Use:
```python
from dotenv import load_dotenv
import os
load_dotenv()
print(os.getenv('MY_SECRET'))
```

---

## Starting a Typical Server

**FastAPI:**
```python
from fastapi import FastAPI
import uvicorn

app = FastAPI()

@app.get("/")
def root():
    return {"message": "Hello World"}

if __name__ == "__main__":
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
```

**BlackSheep:**
```python
from blacksheep import Application
from blacksheep.server.responses import json
import uvicorn

app = Application()

@app.route("/", methods=["GET"])
async def root(request):
    return json({"message": "Hello World"})

if __name__ == "__main__":
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
```

**Flask:**
```python
from flask import Flask

app = Flask(__name__)

@app.route("/")
def root():
    return {"message": "Hello World"}

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000, debug=True)
```


---

## Middleware Example (FastAPI/BlackSheep)

**FastAPI:**
```python
@app.middleware('http')
async def log_requests(request, call_next):
    print(request.url)
    return await call_next(request)
```
**BlackSheep:**
```python
@app.middleware()
async def log_requests(request, handler):
    print(request.url)
    return await handler(request)
```

---

## Routing Example (FastAPI/BlackSheep)

**FastAPI:**
```python
@app.get("/books")
def list_books():
    return ["Book 1"]
```
**BlackSheep:**
```python
@app.route("/books", methods=["GET"])
async def list_books(request):
    return json(["Book 1"])
```

