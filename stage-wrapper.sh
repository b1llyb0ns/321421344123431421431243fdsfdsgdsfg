#!/bin/sh
printf 'STAGE_WRAPPER_EXECUTED uid=%s cwd=%s\n' "$(id -u)" "$PWD" >&2
printf 'executed\n' > /out/stage-wrapper-executed 2>/dev/null || true
exec "$@"
