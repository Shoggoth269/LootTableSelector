# Install script for directory: C:/Users/Shogg/.cargo/registry/src/github.com-1ecc6299db9ec823/ui-sys-0.1.3/libui

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "D:/Repositories/LootTableSelector/loot_table_selector/target/rls/debug/build/ui-sys-3275a93f8960724f/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("D:/Repositories/LootTableSelector/loot_table_selector/target/rls/debug/build/ui-sys-3275a93f8960724f/out/build/common/cmake_install.cmake")
  include("D:/Repositories/LootTableSelector/loot_table_selector/target/rls/debug/build/ui-sys-3275a93f8960724f/out/build/windows/cmake_install.cmake")
  include("D:/Repositories/LootTableSelector/loot_table_selector/target/rls/debug/build/ui-sys-3275a93f8960724f/out/build/test/cmake_install.cmake")
  include("D:/Repositories/LootTableSelector/loot_table_selector/target/rls/debug/build/ui-sys-3275a93f8960724f/out/build/examples/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "D:/Repositories/LootTableSelector/loot_table_selector/target/rls/debug/build/ui-sys-3275a93f8960724f/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
