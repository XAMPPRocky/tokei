// 43 lines 31 code 8 comments 4 blanks
Shader "Custom/Sample shader"
{
    Properties
    {
        _MainTex ("Texture", 2D) = "white" {}
    }
    SubShader
    {
        Tags { "Queue"="Transparent" "RenderType"="Transparent" }

        // blending
        Blend SrcAlpha OneMinusSrcAlpha
        /*

          multi-line comment

        */
        Pass
        {
            CGPROGRAM
            #pragma vertex vert

            struct appdata
            {
                float4 vertex : POSITION;
                float2 uv : TEXCOORD0;
            };

            sampler2D _MainTex;

            // vertex
            v2f vert (appdata v)
            {
                v2f o;
                o.vertex = UnityObjectToClipPos(v.vertex);
                o.uv = TRANSFORM_TEX(v.uv, _MainTex);
                return o;
            }
            ENDCG
        }
    }
}
