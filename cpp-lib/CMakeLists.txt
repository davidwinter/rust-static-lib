cmake_minimum_required(VERSION 3.10)
project(GreetingLibrary)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_library(greeting STATIC src/greeting.cpp)

target_include_directories(greeting PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
