SET(HS ${CMAKE_CURRENT_SOURCE_DIR}/heatshrink)
INCLUDE_DIRECTORIES( ${HS})
SET(HS_SRCS ${HS}/heatshrink_decoder.c)

ADD_LIBRARY(simplerILI9341 
            simpler9341.cpp 
            simpler9341_ex.cpp 
            simpler9341_ex_ll.cpp 
            simpler9341_initCommand.cpp
            
            ${ILI9341_DRIVER})
include_directories(${CMAKE_CURRENT_SOURCE_DIR})

ADD_DEFINITIONS("-O2")
ADD_LIBRARY(simplerILI9341HS STATIC      ${HS_SRCS})
            
IF("${LN_ARCH}" STREQUAL "RISCV")
    MESSAGE(STATUS "ILI9341 : Using PB8 instead of PB3 for LCD")
    TARGET_COMPILE_OPTIONS(simplerILI9341 PRIVATE "-DUSE_PB8_INSTEAD_OF_PB3")
ENDIF()
