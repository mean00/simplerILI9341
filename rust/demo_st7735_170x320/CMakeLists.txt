include(rnCmake)
#corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/lnbmp/Cargo.toml          CFLAGS ${LN_LTO_RUST_FLAGS} )
IF(USE_RP2040)
  SET(FTR "rp2040")
ENDIF()
RUST_ADD( rsdemo ${CMAKE_CURRENT_SOURCE_DIR}/rs/Cargo.toml ${FTR})


