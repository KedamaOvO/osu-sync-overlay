#version 100
precision mediump float;

uniform sampler2D Texture;
varying vec2 Frag_UV;
varying vec4 Frag_Color;

void main()
{
    gl_FragColor = Frag_Color;
    gl_FragColor.a = texture2D(Texture, Frag_UV.st).a;
}