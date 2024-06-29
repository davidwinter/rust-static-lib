cmake_minimum_required(VERSION 3.10)
project(HelloLibraryUser)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_executable(hello_user src/main.cpp)

target_link_libraries(hello_user PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../cpp-lib/build/libgreeting.a)
