cmake_minimum_required(VERSION 3.10)

project(process)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=gnu11")

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(SOURCE_DIR "${PROJECT_SOURCE_DIR}/src")
file(GLOB SOURCE
        "${SOURCE_DIR}/*.c"
        "${SOURCE_DIR}/**/*.c"
        )
add_library(${PROJECT_NAME} SHARED ${SOURCE})
