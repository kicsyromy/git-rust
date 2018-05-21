cmake_minimum_required (VERSION 3.9)

set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/core)

ExternalProject_Add(
    core
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build --release
    BINARY_DIR "${CMAKE_CURRENT_SOURCE_DIR}"
    INSTALL_COMMAND ""
    LOG_BUILD ON)
