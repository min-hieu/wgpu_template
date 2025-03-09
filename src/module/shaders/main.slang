struct VertexOut {
  float4 color : COLOR0;
  float4 position : SV_POSITION;
}

struct Uniforms {
  float angle;
}

[[vk_binding(0,0)]]
ConstantBuffer<Uniforms> uniforms;

static float2[3] v_positions = float2[3](
  float2(0.0, 1.0),
  float2(1.0, -1.0),
  float2(-1.0, -1.0)
);

static float4[3] v_colors = float4[3](
  float4(1.0, 0.0, 0.0, 1.0),
  float4(0.0, 1.0, 0.0, 1.0),
  float4(0.0, 0.0, 1.0, 1.0)
);

[shader("vertex")]
VertexOut vs_main(uint vertexId : SV_VertexID) {
  VertexOut out;

  out.position = float4(v_positions[vertexId], 0.0, 1.0);
  out.position.x = out.position.x * cos(uniforms.angle);
  out.color = v_colors[vertexId];

  return out;
}

[shader("fragment")]
float4 fs_main(VertexOut in) : SV_TARGET {
  return in.color;
}
