#!/usr/bin/env bash
# Prints free disk space in GB (no unit suffix)
df -BG / | awk 'NR==2 {gsub(/G/, "", $4); print $4}'
