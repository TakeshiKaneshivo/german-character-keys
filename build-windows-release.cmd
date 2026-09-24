@echo off
setlocal EnableExtensions
cd /d "%~dp0"

echo Building German Character Keys for US Keyboards (ÄÖÜß) for Windows x64...

where node >nul 2>&1
if errorlevel 1 (
  echo Error: Node.js is required but was not found in PATH.
  exit /b 1
)
where npm >nul 2>&1
if errorlevel 1 (
  echo Error: npm is required but was not found in PATH.
  exit /b 1
)
where cargo >nul 2>&1
if errorlevel 1 (
  echo Error: Rust Cargo is required but was not found in PATH.
  exit /b 1
)

set "BUILD_ARCH=%PROCESSOR_ARCHITEW6432%"
if not defined BUILD_ARCH set "BUILD_ARCH=%PROCESSOR_ARCHITECTURE%"
if /i not "%BUILD_ARCH%" == "AMD64" (
  echo Error: this script requires Windows x64. Detected: %BUILD_ARCH%
  exit /b 1
)

if not exist "node_modules\.bin\tauri.cmd" (
  echo Installing JavaScript dependencies...
  call npm ci
  if errorlevel 1 exit /b 1
)

call npm run build:windows-release
if errorlevel 1 (
  echo.
  echo Windows Release build failed.
  exit /b 1
)

if not exist "src-tauri\target\release\german-character-keys.exe" (
  echo Error: Release application was not generated.
  exit /b 1
)
for /f "usebackq delims=" %%P in (`node -e "const fs=require('fs');const packageJson=JSON.parse(fs.readFileSync('package.json','utf8'));const config=JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json','utf8'));const directory='src-tauri/target/release/bundle/nsis/';const source=directory+config.productName+'_'+packageJson.version+'_x64-setup.exe';const destination=directory+config.app.windows[0].title+'_'+packageJson.version+'_x64-setup.exe';if(!fs.existsSync(source))process.exit(2);if(fs.existsSync(destination))fs.unlinkSync(destination);fs.renameSync(source,destination);process.stdout.write(destination)"`) do set "NAMED_INSTALLER=%%P"
if errorlevel 1 (
  echo Error: NSIS installer was not generated or could not be renamed.
  exit /b 1
)
if not defined NAMED_INSTALLER (
  echo Error: Could not determine the final installer path.
  exit /b 1
)

echo.
echo Windows x64 Release build completed.
echo Installer: %NAMED_INSTALLER%
echo App EXE:   src-tauri\target\release\german-character-keys.exe
pause
exit /b 0
