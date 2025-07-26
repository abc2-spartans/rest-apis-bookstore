#!/bin/sh
source .venv/bin/activate
exec uvicorn main:app --port 5000 --reload