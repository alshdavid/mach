#version 300 es

#pragma glslify: test = require('./lib.glsl')

precision mediump float;

void main() {
  test();
}
