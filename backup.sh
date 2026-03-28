#!/bin/bash
for IP in 192.168.99.1 192.168.99.2 192.168.99.3; do
    ssh admin@$IP "show running-config" > backup_$IP.txt
done