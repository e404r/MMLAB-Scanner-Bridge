@echo off
chcp 65001 >nul
echo ========================================================
echo   MMLAB Scanner Bridge — Windows Build Script
echo ========================================================

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust / Cargo ვერ მოიძებნა!
    echo გთხოვთ დააინსტალიროთ Rust: https://rustup.rs/
    pause
    exit /b 1
)

echo [1/2] იწყება ოპტიმიზებული კომპილაცია (Release)...
cargo build --release

if %errorlevel% equ 0 (
    echo.
    echo [2/2] მზადდება შესრულებადი ფაილი...
    if not exist "dist" mkdir "dist"
    if not exist "dist\Windows" mkdir "dist\Windows"
    copy "target\release\hr-scanner-bridge.exe" "dist\Windows\MMLAB Scanner Bridge.exe" >nul
    copy "target\release\hr-scanner-bridge.exe" "MMLAB Scanner Bridge.exe" >nul
    
    echo.
    echo ========================================================
    echo   წარმატებით დაკომპილირდა!
    echo   ფაილი: dist\Windows\MMLAB Scanner Bridge.exe
    echo   (ასევე დაკოპირდა ძირეულ საქაღალდეში)
    echo ========================================================
) else (
    echo.
    echo [ERROR] კომპილაციის შეცდომა!
)

pause
