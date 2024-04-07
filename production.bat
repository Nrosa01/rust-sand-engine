@echo off

if "%1" == "build" (
   cargo build --target wasm32-unknown-unknown --profile production
) else if "%1" == "run" (
   basic-http-server .
) else (
   cargo build --target wasm32-unknown-unknown --profile production
)