include(rnCmake)
#corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/lnbmp/Cargo.toml          CFLAGS ${LN_LTO_RUST_FLAGS} )

RUST_ADD( lnspi_ili9341 ${CMAKE_CURRENT_SOURCE_DIR}/rs/Cargo.toml)


