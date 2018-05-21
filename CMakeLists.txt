cmake_minimum_required (VERSION 3.9)
project (git-rust)

set (CMAKE_CXX_STANDARD 17)
set (CMAKE_CXX_STANDARD_REQUIRED)
set (CMAKE_CXX_EXTENSIONS OFF)

set (SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/src)
set (INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

file (GLOB SOURCES ${SOURCE_DIR}/*.cpp)
file (GLOB INCLUDE ${INCLUDE_DIR}/*.h)

include(ExternalProject)

add_subdirectory (${CMAKE_CURRENT_SOURCE_DIR}/subprojects/core)
add_executable (${PROJECT_NAME} ${SOURCES} ${INCLUDE})
add_dependencies (${PROJECT_NAME} core)
target_include_directories (${PROJECT_NAME} PRIVATE ${SOURCE_DIR})
target_link_libraries (${PROJECT_NAME} pthread dl optimized "${CMAKE_CURRENT_SOURCE_DIR}/subprojects/core/target/release/libcore.a")
