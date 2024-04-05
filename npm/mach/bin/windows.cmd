@echo off
SET dp0=%~dp0

IF '%MACH_BIN_OVERRIDE%' NEQ '' (
  "%MACH_BIN_OVERRIDE%" %*
  EXIT 0
)

IF '%PROCESSOR_ARCHITECTURE%' EQU 'AMD64' (
  "%dp0%\mach-windows-amd64\bin\mach.exe" %*
  EXIT 0
)

IF '%PROCESSOR_ARCHITECTURE%' EQU 'ARM64' (
  "%dp0%\mach-windows-arm64\bin\mach.exe" %*
  EXIT 0
)

echo "Could not find Mach binary for your system. Please compile from source"
echo "Override the built in binary by setting the MACH_BIN_OVERRIDE environment variable"
EXIT 1