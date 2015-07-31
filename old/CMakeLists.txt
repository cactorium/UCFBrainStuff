cmake_minimum_required(VERSION 2.7)

set(CMAKE_BINARY_DIR ${CMAKE_SOURCE_DIR})
 
set(EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR})
set(LIBRARY_OUTPUT_PATH ${CMAKE_BINARY_DIR})
 
# NOTE: These two lines are probably very hackish
set(PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR}/include ${PROJECT_SOURCE_DIR}/emokit/include)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/emokit/cmake_modules/")
 
include_directories("${PROJECT_INCLUDE_DIR}")
include_directories("${PROJECT_SOURCE_DIR}")

add_subdirectory(emokit)
add_subdirectory(libbrain)
