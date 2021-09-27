ADD_LIBRARY(simplerILI9341 
            simpler9341.cpp 
            simpler9341_ex.cpp 
            simpler9341_ex_ll.cpp 
            ${ILI9341_DRIVER})
include_directories(${CMAKE_CURRENT_SOURCE_DIR})
