cmake_minimum_required(VERSION 3.30)
project(RDP_Monitor_UI C)


set(exe "ui")

# Specify the raylib include and library directories
# Change the following dirs according to your paths
include_directories("${PROJECT_SOURCE_DIR}/raylib/build/raylib/include")
set(RAYLIB_LIB "${PROJECT_SOURCE_DIR}/raylib/build/raylib/Release/raylib.lib")

# Add the source files
set(SOURCES main.c funcs.c)

# Add the executable
add_executable(${exe} ${SOURCES})
# define directory to store the bin file
set_target_properties(${exe} PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
# Link raylib and system libraries
target_link_libraries(${exe} ${RAYLIB_LIB}  gdi32 winmm kernel32)

# Suppress deprecation warnings
add_definitions(-D_CRT_SECURE_NO_WARNINGS)
