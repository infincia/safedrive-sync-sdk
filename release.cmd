ECHO "building %PLATFORM%"

rmdir /s /q dist-win-%PLATFORM%-vs2015

set SODIUM_LIB_DIR=dep-win-%PLATFORM%-vs2015\lib
set SODIUM_STATIC=""

set OPENSSL_DIR=%CD%\dep-win-%PLATFORM%-vs2015
set OPENSSL_STATIC=""

set RUSTFLAGS=""

cargo.exe build --release --verbose

mkdir dist-win-%PLATFORM%-vs2015
mkdir dist-win-%PLATFORM%-vs2015\lib
mkdir dist-win-%PLATFORM%-vs2015\include

copy target\release\libsdsync.lib dist-win-%PLATFORM%-vs2015\lib\

copy dep-win-%PLATFORM%-vs2015\lib\* dist-win-%PLATFORM%-vs2015\lib\
copy dep-win-%PLATFORM%-vs2015\include\* dist-win-%PLATFORM%-vs2015\include\
