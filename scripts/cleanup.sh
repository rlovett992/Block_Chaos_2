#!/usr/bin/env bash
set -euo pipefail

DM_NAME="${1:-chaos_delay}"

echo "== Cleanup Script =="
echo "Removing device-mapper target: ${DM_NAME}"

if suod dmsetup ls | grep -q "^${DM_NAME}\b"; then
	sudo dmsetup remove "${DM_NAME}"
	echo "Removed ${DM_NAME}"
else
	echo "No dm device named ${DM_NAME} found"
fi

echo "Done"
