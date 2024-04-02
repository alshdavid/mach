@echo off
@SET TARGET_PATH=
FOR /F %%I IN ('node -e "const { dirname } = require(`path`); console.log(dirname(dirname(require.resolve(`@alshdavid/mach`))))"') DO @SET "TARGET_PATH=%%I"
"%TARGET_PATH%\mach\bin\mach.exe" %*
