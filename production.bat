@echo off

if "%1" == "build" (
   cargo +nightly  build -Z build-std --target x86_64-pc-windows-msvc --profile production
) else if "%1" == "run" (
   cargo +nightly  run -Z build-std --target x86_64-pc-windows-msvc --profile production
) else (
   cargo +nightly  build -Z build-std --target x86_64-pc-windows-msvc --profile production
)