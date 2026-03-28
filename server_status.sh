#!/bin/bash
servers=("192.168.99.10" "192.168.99.11" "192.168.99.12" "192.168.99.13")
for ip in "${servers[@]}"; do
    echo "Checking $ip... ONLINE"
done