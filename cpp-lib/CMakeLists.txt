cmake_minimum_required(VERSION 3.10)
project(GreetingLibrary)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_library(greeting STATIC src/greeting.cpp)

set_target_properties(
    greeting
    PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY $<1:${CMAKE_BINARY_DIR}/lib>
    ARCHIVE_OUTPUT_DIRECTORY $<1:${CMAKE_BINARY_DIR}/lib>
    RUNTIME_OUTPUT_DIRECTORY $<1:${CMAKE_BINARY_DIR}/lib>
)

target_include_directories(greeting PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
