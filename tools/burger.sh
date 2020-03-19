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
print_usage_and_exit() {
cat <<-USAGE 1>&2
Usage   : ${0##*/} [options] [path_to_disk_image]
Options : -c
            Check prerequisites & install components
          -b
            Build Krabs
        ---------------------------------------------------
          -k ELF_file
            Set the 32bit/64bit ELF kernel you want to boot.
          -i initrd_file
            Set initrd.
          -p "command line"
            Set the kernel command line like "root=ram"
USAGE
  exit 1
}

check_file() {
    if [  ! -f "$1"  ]; then
        error_exit 1 "${1##*/} not found."
    fi
}

# === check commands ===========================================================
for cmd in sh bc bzip2 wc awk sed printf cat od ; do
  if ! command -v ${cmd} >/dev/null 2>&1 ; then
    echo "required program '${cmd}' not found" >&2
    exit 1 
  fi
done

################################################################################
# Parse Arguments
################################################################################
# === Print the usage when "--help" is put =====================================
case "$# ${1:-}" in
    '1 -h'|'1 --help'|'1 --version') print_usage_and_exit;;
esac

# === Setting some parameter  ==================================================
# --- 1. Initialize
STAGE3SIZE=''
BINSTAGE3S=''
BZKERNFILE=''
BZKERNSIZE=''
BINBZKERNS=''
INITRDFILE=''
BININITRDS=''
# --- 2. get opts
while getopts cbk:i:p: OPT
do
    case $OPT in
        c)  ${KRABS_DIR}/tools/check.sh
            exit 0
            ;;
        b)  ${KRABS_DIR}/tools/xbuild.sh
            exit 0
            ;;
        k)  ELF_KERNEL="${OPTARG}"
            check_file ${ELF_KERNEL}
            bzip2 -zcfk1 "${ELF_KERNEL}" > "${ELF_KERNEL}.bz2"
            BZKERNFILE="${ELF_KERNEL}.bz2"
            BZKERNSIZE="$(cat ${BZKERNFILE} | wc -c | awk '{print $1}' | sed 's:$:/512+1:' | bc)"
            BINBZKERNS="$(printf '\%03o\%03o\n' $((${BZKERNSIZE} & 0xFF)) $(((${BZKERNSIZE} & 0xFF00)>>8)))"
            ;;
        i)  INITRDFILE="${OPTARG}"
            check_file $INITRDFILE
            INITRDSIZE="$(cat ${INITRDFILE} | wc -c | awk '{print $1}' | sed 's:$:/512+1:' | bc)"
            BININITRDS="$(printf '\%03o\%03o\n' $((${INITRDSIZE} & 0xFF)) $(((${INITRDSIZE} & 0xFF00)>>8)))"
            ;;
        p)  COMANDLINE="${OPTARG}"
            if [ "${#COMANDLINE}" -gt 120 ]; then
                error_exit 1 'cmdline is too long. it should be under 120 bytes.'
            fi
            ;;
    esac
done
shift $((OPTIND - 1))
# --- 3. set DISKIMAGE & extract boot patition sector
case "$#" in
    0)  print_usage_and_exit
        ;;
    1)  if [   -f "$1"  ] || 
           [   -b "$1"  ] ||
           [   -c "$1"  ] || 
           [   -p "$1"  ]; then
          DISKIMAGE=$1
        fi
        ;;
    *)  print_usage_and_exit
        ;;
esac
BOOTPART="$(od -Ax -tx1 -j0x1bE -N64 -v ${DISKIMAGE} | 
awk '$2~/80/{print $13 $12 $11 $10}'                 | 
tr '[:lower:]' '[:upper:]'                           |
sed 's/.*/obase=10; ibase=16; &/'                    | 
bc                                                  )"
if [ -z "${BOOTPART:-}" ]; then
    error_exit 1 "There is no boot partition in ${DISKIMAGE}"
fi

{
check_file "${KRABS_DIR}/target/i586-stage_1st/release/stage_1st.bin"
printf "== Writing stage_1st into Disk image. ==\n"
dd if=${KRABS_DIR}/target/i586-stage_1st/release/stage_1st.bin of="${DISKIMAGE}" conv=notrunc 2>&1


check_file "${KRABS_DIR}/target/i586-stage_2nd/release/stage_2nd.bin"
printf "\n== Writing stage_2nd into Disk image. ==\n"
dd if=${KRABS_DIR}/target/i586-stage_2nd/release/stage_2nd.bin of="${DISKIMAGE}" bs=512 seek=1 conv=notrunc 2>&1
STAGE2SIZE="$(cat ${KRABS_DIR}/target/i586-stage_2nd/release/stage_2nd.bin | wc -c | awk '{print $1}' | sed 's:$:/512+1:' | bc)"
BINSTAGE2S="$(printf '\%03o\%03o\n' $((${STAGE2SIZE} & 0xFF)) $(((${STAGE2SIZE} & 0xFF00)>>8)))"
printf "${BINSTAGE2S}" | dd of="${DISKIMAGE}" bs=1 seek=$((0x1bc)) count=2 conv=notrunc 2>&1


check_file "${KRABS_DIR}/target/i586-stage_3rd/release/stage_3rd.bin"
printf "\n== Writing stage_3rd into Disk image. ==\n"
dd if=${KRABS_DIR}/target/i586-stage_3rd/release/stage_3rd.bin of="${DISKIMAGE}" bs=512 seek="${BOOTPART}" conv=notrunc 2>&1
STAGE3SIZE="$(cat ${KRABS_DIR}/target/i586-stage_3rd/release/stage_3rd.bin | wc -c | awk '{print $1}' | sed 's:$:/512+1:' | bc)"
BINSTAGE3S="$(printf '\%03o\%03o\n' $((${STAGE3SIZE} & 0xFF)) $(((${STAGE3SIZE} & 0xFF00)>>8)))"
printf "${BINSTAGE3S}" | dd of="${DISKIMAGE}" bs=1 seek=$((0x278)) count=2 conv=notrunc 2>&1


check_file "${KRABS_DIR}/target/i586-stage_4th/release/stage_4th.bin"
printf "\n== Writing stage_4th into Disk image. ==\n"
dd if=${KRABS_DIR}/target/i586-stage_4th/release/stage_4th.bin of="${DISKIMAGE}" bs=512 seek="$((${BOOTPART}+${STAGE3SIZE}))" conv=notrunc 2>&1
STAGE4SIZE="$(cat target/i586-stage_4th/release/stage_4th.bin | wc -c | awk '{print $1}' | sed 's:$:/512+1:' | bc)"
BINSTAGE4S="$(printf '\%03o\%03o\n' $((${STAGE4SIZE} & 0xFF)) $(((${STAGE4SIZE} & 0xFF00)>>8)))"
printf "${BINSTAGE4S}" | dd of="${DISKIMAGE}" bs=1 seek=$((0x27a)) count=2 conv=notrunc 2>&1


# Writing kenrel and initrd, if exists.
if [ -n "${BZKERNFILE:-}" ]; then
    printf "\n== Writing kernel into boot partition. ==\n"
    dd if="${BZKERNFILE}" of="${DISKIMAGE}" bs=512 seek="$((${BOOTPART}+${STAGE3SIZE}+${STAGE4SIZE}))" conv=notrunc 2>&1
    printf "${BINBZKERNS}" | dd of="${DISKIMAGE}" bs=1 seek=$((0x27c)) count=2 conv=notrunc 2>&1
    rm "${BZKERNFILE}"
fi

if [ -n "${INITRDFILE:-}" ]; then
    printf "\n== Writing initrd into boot partition. ==\n"
    dd if="${INITRDFILE}" of="${DISKIMAGE}" bs=512 seek="$((${BOOTPART}+${STAGE3SIZE}+${STAGE4SIZE}+${BZKERNSIZE}))" conv=notrunc 2>&1
    printf "${BININITRDS}" | dd of="${DISKIMAGE}" bs=1 seek=$((0x27e)) count=2 conv=notrunc  2>&1
fi

# Writing command line, if exists.
if [ -n "${COMANDLINE:-}" ]; then
    printf "\n== Writing kernel cmdline into boot param table. ==\n"
    printf "${COMANDLINE}" | cut -c 1-120 | dd of="${DISKIMAGE}" bs=1 seek=$((0x200)) count="${#COMANDLINE}" conv=notrunc 2>&1
fi
} | grep -Ev '(the table of contents is empty|is valid for Java but not for C)'

