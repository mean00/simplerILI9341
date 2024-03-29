SET(HS ${CMAKE_CURRENT_SOURCE_DIR}/heatshrink)
INCLUDE_DIRECTORIES( ${HS})
SET(HS_SRCS ${HS}/heatshrink_decoder.c)

OPTION(SIMPLER_ILI9341_FONT_COMPRESSION OFF)
OPTION(SIMPLER_ILI9341_2BPP_SUPPORT     OFF)


IF(SIMPLER_ILI9341_2BPP_SUPPORT)
    SET(SRC_2BPP simpler9341_ex_ll_2bpp.cpp)
ELSE()
    SET(SRC_2BPP simpler9341_ex_ll_2bpp_stubs.cpp)
ENDIF()

IF(SIMPLER_ILI9341_FONT_COMPRESSION)
    ADD_DEFINITIONS("-DILI_ENABLE_COMPRESSION")
ENDIF(SIMPLER_ILI9341_FONT_COMPRESSION)

ADD_LIBRARY(simplerILI9341 
            simpler9341.cpp 
            simpler9341_ex.cpp 
            simpler9341_ex_ll.cpp  
            simpler9341_initCommand.cpp
            simpler9341_ex_ll_1bpp.cpp
            simpler9341_geometry.cpp
            simpler9341_adv.cpp
            ${SRC_2BPP}     
            ${ILI9341_DRIVER})
include_directories(${CMAKE_CURRENT_SOURCE_DIR})

IF(SIMPLER_ILI9341_FONT_COMPRESSION)
    ADD_DEFINITIONS("-O2")
    ADD_LIBRARY(simplerILI9341HS STATIC      ${HS_SRCS})
ENDIF()
            
IF("${LN_ARCH}" STREQUAL "RISCV")
    MESSAGE(STATUS "ILI9341 : Using PB8 instead of PB3 for LCD")
    TARGET_COMPILE_OPTIONS(simplerILI9341 PRIVATE "-DUSE_PB8_INSTEAD_OF_PB3")
ENDIF()

target_link_libraries( simplerILI9341 lnArduino )