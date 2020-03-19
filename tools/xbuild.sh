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

# === Define the functions =====================================================
error_exit() {
    ${2+:} false && echo "${0##*/}: $2" 1>&2
    exit $1
}

# === Build stages =============================================================
{
printf "== Building stage_1st. ==\n"
cd ${KRABS_DIR}/src/stage_1st
if cargo xbuild --release; then
    cargo objcopy -- -I elf32-i386 -O binary \
        ${KRABS_DIR}/target/i586-stage_1st/release/stage_1st \
        ${KRABS_DIR}/target/i586-stage_1st/release/stage_1st.bin
else
    cd $KRABS_DIR
    error_exit 1 'stage_1st build failed'
fi 2>&1
cd $KRABS_DIR


printf "\n== Building stage_2nd. ==\n"
cd ${KRABS_DIR}/src/stage_2nd
if cargo xbuild --release; then
    cargo objcopy -- -I elf32-i386 -O binary \
        ${KRABS_DIR}/target/i586-stage_2nd/release/stage_2nd \
        ${KRABS_DIR}/target/i586-stage_2nd/release/stage_2nd.bin
else
    cd $KRABS_DIR
    error_exit 1 'stage_2nd build failed'
fi 2>&1
cd $KRABS_DIR


printf "\n== Building stage_3rd. ==\n"
cd ${KRABS_DIR}/src/stage_3rd
if cargo xbuild --release; then
    cargo objcopy -- -I elf32-i386 -O binary \
        ${KRABS_DIR}/target/i586-stage_3rd/release/stage_3rd \
        ${KRABS_DIR}/target/i586-stage_3rd/release/stage_3rd.bin
else
    cd $KRABS_DIR
    error_exit 1 'stage_3rd build failed'
fi 2>&1
cd $KRABS_DIR


printf "\n== Building stage_4th. ==\n"
cd ${KRABS_DIR}/src/stage_4th
if cargo xbuild --release; then
    cargo objcopy -- -I elf32-i386 -O binary \
        ${KRABS_DIR}/target/i586-stage_4th/release/stage_4th \
        ${KRABS_DIR}/target/i586-stage_4th/release/stage_4th.bin
else
    cd $KRABS_DIR
    error_exit 1 'stage_4th build failed'
fi 2>&1
cd $KRABS_DIR

} | grep -Ev '(the table of contents is empty|is valid for Java but not for C|There is no root package to read)'

