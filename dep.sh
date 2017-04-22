#!/usr/bin/env bash

set -e

if [ -z "${TARGET}" ]; then
    export TARGET=`rustup show | awk 'match($0, /Default host: ([0-9a-zA-Z\_]).+/) { ver = substr($3, RSTART, RLENGTH); print ver;}'`
fi

export BUILD_PREFIX=${PWD}/dep/${TARGET}
export DIST_PREFIX=${PWD}/dist-${TARGET}

export PATH=${BUILD_PREFIX}/bin:${PATH}

mkdir -p ${BUILD_PREFIX}/lib
mkdir -p ${BUILD_PREFIX}/include
mkdir -p ${BUILD_PREFIX}/bin

mkdir -p ${DIST_PREFIX}/lib
mkdir -p ${DIST_PREFIX}/include
mkdir -p ${DIST_PREFIX}/bin

mkdir -p src
mkdir -p build

# these are at the top for visibility, changing a version will always cause a rebuild, otherwise
# they will only be rebuilt if the built product is not found
export SODIUM_VER=1.0.12
export SODIUM_VER_FILE=${BUILD_PREFIX}/.sodium_ver
export SODIUM_ARGS="--enable-shared=no"

export LIBDBUS_VER=1.10.14
export LIBDBUS_VER_FILE=${BUILD_PREFIX}/.dbus_ver
export LIBDBUS_ARGS="--enable-shared=no --disable-tests --with-x=no --disable-systemd --disable-launchd --disable-libaudit --disable-selinux --disable-apparmor"

export EXPAT_VER=2.2.0
export EXPAT_VER_FILE=${BUILD_PREFIX}/.expat_ver
export EXPAT_ARGS="--enable-shared=no"

export BUILD_DBUS=true
export BUILD_LIBSODIUM=true

export RUSTFLAGS=""
export CFLAGS="-O2 -g -I${BUILD_PREFIX}/include"
export CPPFLAGS="-O2 -g -I${BUILD_PREFIX}/include"
export LDFLAGS="-flto -L${BUILD_PREFIX}/lib"

export OSX_VERSION_MIN="10.9"
export OSX_CPU_ARCH="core2"
export MAC_ARGS="-arch x86_64 -mmacosx-version-min=${OSX_VERSION_MIN} -march=${OSX_CPU_ARCH}"

case ${TARGET} in
    x86_64-apple-darwin)
        export CFLAGS="${CFLAGS} ${MAC_ARGS}"
        export CPPFLAGS="${CPPFLAGS} ${MAC_ARGS}"
        export LDFLAGS="${LDFLAGS} ${MAC_ARGS}"
        export RUSTFLAGS="${RUSTFLAGS} -C link-args=-mmacosx-version-min=${OSX_VERSION_MIN}"
        ;;
    x86_64-unknown-linux-gnu)
        export CFLAGS="${CFLAGS}"
        export CPPFLAGS="${CPPFLAGS}"
        export LDFLAGS="${LDFLAGS}"
        ;;
    i686-unknown-linux-gnu)
        export CFLAGS="${CFLAGS} -m32"
        export CPPFLAGS="${CPPFLAGS} -m32"
        export LDFLAGS="${LDFLAGS}"
        export PKG_CONFIG_ALLOW_CROSS=1
        ;;
    x86_64-unknown-linux-musl)
        export CFLAGS="${CFLAGS}"
        export CPPFLAGS="${CPPFLAGS}"
        export LDFLAGS="${LDFLAGS}"
        export CC=musl-gcc
        ;;
    i686-unknown-linux-musl)
        export CFLAGS="${CFLAGS} -m32"
        export CPPFLAGS="${CPPFLAGS} -m32"
        export LDFLAGS="${LDFLAGS}"
        export CC=musl-gcc
        export PKG_CONFIG_ALLOW_CROSS=1
        ;;
    *)
        ;;
esac

if [ ! -f dep/${TARGET}/lib/libdbus-1.a ] || [ ! -f ${LIBDBUS_VER_FILE} ] || [ ! $(<${LIBDBUS_VER_FILE}) = ${LIBDBUS_VER} ]; then
    if [ ${BUILD_DBUS} = true ]; then

        echo "Building libexpat ${EXPAT_VER} for ${TARGET} in ${BUILD_PREFIX}"


        curl -L https://downloads.sourceforge.net/project/expat/expat/${EXPAT_VER}/expat-${EXPAT_VER}.tar.bz2 -o expat-${EXPAT_VER}.tar.bz2 > /dev/null
        tar xf expat-${EXPAT_VER}.tar.bz2 > /dev/null
        pushd expat-${EXPAT_VER}
        ./configure --prefix=${BUILD_PREFIX} ${EXPAT_ARGS} > /dev/null
        make > /dev/null
        make install > /dev/null
        popd
        rm -rf expat*
        echo ${EXPAT_VER} > ${EXPAT_VER_FILE}

        echo "Building libdbus ${LIBDBUS_VER} for ${TARGET} in ${BUILD_PREFIX}"

        curl -L https://dbus.freedesktop.org/releases/dbus/dbus-${LIBDBUS_VER}.tar.gz -o dbus-${LIBDBUS_VER}.tar.gz> /dev/null
        tar xf dbus-${LIBDBUS_VER}.tar.gz > /dev/null
        pushd dbus-${LIBDBUS_VER}
        ./configure --prefix=${BUILD_PREFIX} ${LIBDBUS_ARGS} > /dev/null
        make > /dev/null
        make install > /dev/null
        popd
        rm -rf dbus*
        echo ${LIBDBUS_VER} > ${LIBDBUS_VER_FILE}
    else
        echo "Not set to build libdbus"
    fi
else
    echo "Not building libdbus"
fi

if [ ! -f dep/${TARGET}/lib/libsodium.a ] || [ ! -f ${SODIUM_VER_FILE} ] || [ ! $(<${SODIUM_VER_FILE}) = ${SODIUM_VER} ]; then
    if [ ${BUILD_LIBSODIUM} = true ]; then

        echo "Building libsodium ${SODIUM_VER} for ${TARGET} in ${BUILD_PREFIX}"

        curl -L https://github.com/jedisct1/libsodium/releases/download/${SODIUM_VER}/libsodium-${SODIUM_VER}.tar.gz -o libsodium-${SODIUM_VER}.tar.gz > /dev/null
        tar xf libsodium-${SODIUM_VER}.tar.gz > /dev/null
        pushd libsodium-${SODIUM_VER}
        ./configure --prefix=${BUILD_PREFIX} ${SODIUM_ARGS} > /dev/null
        make clean > /dev/null
        make > /dev/null
        make install > /dev/null
        popd
        rm -rf libsodium*
        echo ${SODIUM_VER} > ${SODIUM_VER_FILE}

    else
        echo "Not set to build libsodium"
    fi
else
    echo "Not building libsodium"
fi