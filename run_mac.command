#!/bin/bash
cd "$(dirname "$0")"

# Open browser after 1 second
(sleep 1 && open "http://localhost:8000" || xdg-open "http://localhost:8000") &

# Start Python server
echo "Starting Local Server..."
python3 -m http.server 8000
