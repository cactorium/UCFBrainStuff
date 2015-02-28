project(brain)

list(APPEND PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR}/src)
set(LIBRARY_OUTPUT_PATH ${CMAKE_BINARY_DIR}/lib)

add_library(brain src/libbrain.c)
add_dependencies(brain emokit)

