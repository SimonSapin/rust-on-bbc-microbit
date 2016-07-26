include(ExternalProject)

ExternalProject_Add(
    rust
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build --target=cortex-m0 --release
    BINARY_DIR "${CMAKE_CURRENT_LIST_DIR}/.."
    INSTALL_COMMAND ""
    USES_TERMINAL_BUILD 1
    BUILD_ALWAYS 1
    BUILD_BYPRODUCTS "${CMAKE_CURRENT_LIST_DIR}/../target/cortex-m0/release/librust.a")

add_dependencies(minimal rust)

target_link_libraries(minimal "${CMAKE_CURRENT_LIST_DIR}/../target/cortex-m0/release/librust.a")
