cmake_minimum_required(VERSION 3.24)
project(<%= project_name %>)

set(CMAKE_CXX_STANDARD 20)

include(FetchContent)
FetchContent_Declare(
       googletest
       URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)

FETCHCONTENT_MAKEAVAILABLE(googletest)

enable_testing()

add_executable(<%= project_name %> main.cpp)
target_link_libraries(<%= project_name %> GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(<%= project_name %>)
