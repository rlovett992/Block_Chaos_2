#!/usr/bin/env bash
set -euo pipefail

echo  "== Block I/O Chaos Environment Check =="

echo
echo "[1] Checking required binaries..."
command -v dmsetup >/dev/null || { echo "ERROR: dmsetup not found (install: sudo apt install -y lvm2)"; exit 1; }
command -v fio >/dev/null || { echo "ERROR: fio not found (install: sudo apt install -y fio)"; exit 1; }
command -v losetup >/dev/null || { echo "ERROR: losetup nout found (install: sudo spt install -y util-linux)"; exit 1; }

echo " dmsetup: OK"
echo " fio: OK"
echo " losetup: OK"

echo
echo "[2] Checking device-mapper kernel modules (informational)..."
lsmod | grep -E 'dm_mod|dm_delay|dm_flakey' || echo " (modules may auto-load on demand)"

echo
echo "[3] Checking sudo access..."
sudo -n true 2>/dev/null || echo " NOTE: sudo will prompt for password when running chaos"

echo
echo "Environment looks ready"
