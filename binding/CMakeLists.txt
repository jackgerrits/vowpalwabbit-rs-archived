cmake_minimum_required(VERSION 3.7)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
project(vowpalwabbit-rs-bindings LANGUAGES CXX)

set(STATIC_LINK_VW_JAVA ON CACHE BOOL "" FORCE)
set(VW_INSTALL OFF CACHE BOOL "" FORCE)
set(RAPIDJSON_SYS_DEP OFF CACHE BOOL "" FORCE)
set(FMT_SYS_DEP OFF CACHE BOOL "" FORCE)
set(SPDLOG_SYS_DEP OFF CACHE BOOL "" FORCE)
set(VW_BOOST_MATH_SYS_DEP OFF CACHE BOOL "" FORCE)
set(VW_ZLIB_SYS_DEP OFF CACHE BOOL "" FORCE)

add_subdirectory(external/vowpal_wabbit)

add_library(vw_rs_bindings SHARED src/binding.cc src/binding.h)
target_compile_definitions(vw_rs_bindings PRIVATE VW_RS_BUILDING_DLL)
target_link_libraries(vw_rs_bindings PRIVATE VowpalWabbit::vw)
set_target_properties(vw_rs_bindings PROPERTIES LINK_FLAGS_RELEASE -s)
