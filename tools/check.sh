#!/bin/sh
# An experimental simple build command.
#

# === Initialize Shell Environment =============================================
set -u
umask 0022
unset IFS
export LC_ALL='C'
SCRIPT_DIR="$(cd $(dirname $0); pwd)"
KRABS_DIR="${SCRIPT_DIR%/*}"
cd $KRABS_DIR

echo '== check commands =='
for cmd in sh bc bzip2 wc awk sed printf cat od ; do
  if ! command -v ${cmd} >/dev/null 2>&1 ; then
    echo "    required program '${cmd}' not found" >&2
    exit 1 
  fi
done
echo '    commands check passed'
echo

echo '== check components & install =='
if ! (cargo list | grep xbuild) 2>&1 1>/dev/null; then
    echo '    xbuild not found'
    echo '        installing cargo-xbuild'
    cargo install cargo-xbuild 
    echo '        xbuild installed'
else
    echo '    xbuild installed'
fi

if ! (cargo list | grep objcopy) 2>&1 1>/dev/null; then
    echo '    cargo-binutils not found'
    echo '        installing cargo-binutils'
    cargo install cargo-binutils
    echo '        cargo-binutils installed'
else
    echo '    cargo-binutils installed'
fi

if ! (rustup component list | grep llvm-tools-preview) 2>&1 1>/dev/null; then
    echo '    llvm-tools-preview not found'
    echo '        installing llvm-tools-preview'
    rustup component add llvm-tools-preview
    echo '        llvm-tools-preview installed'
else
    echo '    llvm-tools-preview installed'
fi

if ! (rustup component list | grep rust-src) 2>&1 1>/dev/null; then
    echo '    rust-src not found'
    echo '        installing rust-src'
    rustup component add rust-src
    echo '        rust-src installed'
else
    echo '    rust-src installed'
fi
echo