IF [%ARCH%]==[] set ARCH=x86_64
IF [%TARGET%]==[] set TARGET=x86_64-pc-windows-msvc
IF [%TOOLSET%]==[] set TOOLSET=v141_xp
IF [%LINKTYPE%]==[] set LINKTYPE=static

set LIBPREFIX=
IF [%TOOLSET%]==[v141_xp] set LIBPREFIX=lib

ECHO building SafeDrive for Windows-%ARCH%

del /q dist-%TARGET%-%TOOLSET%-%LINKTYPE%

mkdir dist-%TARGET%-%TOOLSET%-%LINKTYPE%
mkdir dist-%TARGET%-%TOOLSET%-%LINKTYPE%\lib
mkdir dist-%TARGET%-%TOOLSET%-%LINKTYPE%\include
mkdir dist-%TARGET%-%TOOLSET%-%LINKTYPE%\bin

set NATIVE_BUILD_PREFIX=dep\%TARGET%\%TOOLSET%\%LINKTYPE%

set SODIUM_LIB_DIR=%CD%\dep\%TARGET%\%TOOLSET%\%LINKTYPE%\lib
set SODIUM_STATIC=""
set RUST_BACKTRACE="1"
set RUST_FLAGS="-Z print-link-args"

call dep.cmd

call rustver.bat

rustup override set %RUST_VER%

cargo.exe build --release -p libsafedrive --target %TARGET%
cheddar -f "libsafedrive\src\c_api.rs" "dist-%TARGET%-%TOOLSET%-%LINKTYPE%\include\sddk.h"

cargo.exe build --release -p safedrive --target %TARGET%


copy /y "target\%TARGET%\release\safedrive.dll" "dist-%TARGET%-%TOOLSET%-%LINKTYPE%\lib\%LIBPREFIX%safedrive.dll"


copy /y "target\%TARGET%\release\safedrive.exe" "dist-%TARGET%-%TOOLSET%-%LINKTYPE%\bin\"

