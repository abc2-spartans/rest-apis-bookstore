#!/bin/bash

# Development script with live reload
echo "Starting Bookstore API with live reload..."
echo "The API will automatically restart when files change."
echo ""
echo "Available endpoints:"
echo "- Health Check: http://localhost:5000/ or http://localhost:5000/api/v1/health"
echo "- Books API: http://localhost:5000/api/v1/books"
echo "- Swagger UI: http://localhost:5000/swagger"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

dotnet watch run
