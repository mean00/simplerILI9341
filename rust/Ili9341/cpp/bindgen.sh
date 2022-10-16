#!/bin/bas
#
#
bindgen \
   --use-core --no-doc-comments \
   --allowlist-function "heatshrink_decoder_.*" \
     --no-layout-tests       \
      --no-derive-copy   \
       --no-derive-debug    \
    --ctypes-prefix cty \
   --default-enum-style rust  \
    --rustified-enum "HSD.*" \
     --no-copy "HSD.*" \
        --no-debug "HSD.*" \
        --no-default "HSD.*" \
        --no-hash "HSD.*" \
    heatshrink_decoder.h \
    -o hs_decoder_bindings.rs.tmp \
    -- -x c++   
    cat hs_decoder_bindings.rs.tmp  | grep -v "^#\[derive" >  hs_decoder_bindings.rs
exit 0
     --no-debug ".*" --no-derive-debug ".*" \

