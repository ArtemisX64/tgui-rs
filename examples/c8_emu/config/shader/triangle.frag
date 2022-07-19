#version 330

out vec4 color;
uniform vec3 ucolor;

void main(){
	color = vec4(ucolor, 1.0);
}