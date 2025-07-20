#!/bin/sh
source .venv/bin/activate
exec uvicorn main:app --reload --port 5000
