// 15 lines 8 code 5 comments 2 blanks

Texture2D<float4> in_tex;
RWTexture2D<float4> out_tex;

// Blit compute shader
[shader("compute")]
[numthreads(8, 8, 1)]
void main(uint2 id: SV_DispatchThreadID) {
  /*
    Perform the blit
  */
  out_tex[id] = in_tex[id];
  return;
}
