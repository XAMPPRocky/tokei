/* 32 lines 21 code 5 comments 6 blanks */
#include <metal_stdlib>

// comment
struct Uniforms {
    float2 extent;
};

struct VertexIn {
    float2 position [[attribute(0)]];
};

struct VertexOut {
    float2 position [[position]];
};

/*
    multi-line comment
*/

vertex VertexOut vs_main(
    VertexIn in [[stage_in]]
) {
    VertexOut out;
    return out;
}

fragment float4 fs_main(
    VertexOut in [[stage_in]]
) {
    return float4(0.0);
}
