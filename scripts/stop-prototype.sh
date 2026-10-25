#!/usr/bin/env bash
set -euo pipefail
for f in .run/backend.pid .run/frontend.pid;do if test -f "$f";then kill "$(cat "$f")" 2>/dev/null||true;fi;done
