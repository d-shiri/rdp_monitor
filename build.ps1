# Set paths
$CURRENT_LOC = $PWD
$C_SRC = Join-Path $CURRENT_LOC "c_ui"
$C_BUILD = Join-Path $C_SRC "build"
$OUTPUT_DIR = Join-Path $CURRENT_LOC "output_bins"
$RUST_EXE = Join-Path $CURRENT_LOC "target\release\nct.exe"
$C_EXE = Join-Path $C_BUILD "bin\Release\ui.exe"

$OUTPUT_ZIP = "nct.zip"

# Create bin directory
if(Test-Path $OUTPUT_DIR){
    Remove-Item $OUTPUT_DIR -Recurse -Force
}
New-Item -Path $OUTPUT_DIR -ItemType Directory

# Build Rust project
cargo build --release
if ($?) {
    Copy-Item -Path $RUST_EXE -Destination $OUTPUT_DIR
}

# Build C program
#cmake . && cmake --build .\build\ --config Release && mv build\bin\Release\ui.exe ..\bin\.
cmake -S $C_SRC -B $C_BUILD
cmake --build $C_BUILD --config Release
if ($?) {
    Copy-Item -Path $C_EXE -Destination $OUTPUT_DIR
    Copy-Item -Path "$C_SRC\src" -Destination $OUTPUT_DIR -Recurse
}

# Completion message
Write-Host "Build completed!"
Write-Host ""

# Zip files
Write-Host "Zipping files..."
if(Test-Path $OUTPUT_ZIP){
    Remove-Item $OUTPUT_ZIP -Force
}
Compress-Archive -Path "$OUTPUT_DIR\*" -DestinationPath $OUTPUT_ZIP
