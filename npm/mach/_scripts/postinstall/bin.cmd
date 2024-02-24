@echo off
@SET TARGET_PATH=
FOR /F %%I IN ('node -e "console.log(require(`path`).dirname(require.resolve(`@alshdavid/mach`)))"') DO @SET "TARGET_PATH=%%I"
"%TARGET_PATH%\mach\bin\mach.exe" %*
