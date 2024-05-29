#!/bin/bash
shopt -s globstar

function repren() {
    python <(curl https://raw.githubusercontent.com/SteveBeeblebrox/repren/main/repren) "$@"
}

repren -p <(cat << 'EOF'

# Rust literals
"Deno	"system

# JS and TS
Deno.	system.
.Deno	.system

= Deno	= system
Deno {	system {

EOF
) --at-once --literal `find ext runtime | grep -Pv '^\.$|^(?:\.\/)?(?:\.git|'"$(basename "$0")"'$)'`

rm **/*.orig