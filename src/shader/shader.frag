#version 140
#define MAXIMUM_ITERATIONS 250
#define MAXIMUM_DISTANCE_SQUARED 4.0

in vec4 gl_FragCoord;

uniform vec2 julia_param;
uniform vec2 offset;
uniform float zoom;

out vec4 color;

int iter() {
    // [0.0, 800.0] -> [0.0, 4.0] -> [-2.0, 2.0] -> scale and offset
    float re = ((gl_FragCoord.x / 200.0) - 2.0) * zoom + offset.x;
    float im = ((gl_FragCoord.y / 200.0) - 2.0) * zoom + offset.y;

    float re_const = julia_param.x;
    float im_const = julia_param.y;

    float dist2 = re * re + im * im;
    int it = 0;
    while (it < MAXIMUM_ITERATIONS && dist2 < MAXIMUM_DISTANCE_SQUARED) {
        float temp_re = re;

        re = re * re - im * im + re_const;
        im = 2.0 * im * temp_re + im_const;

        dist2 = re * re + im * im;
        it++;
    }

    return it;
}

vec4 make_color() {
    int it = iter();

    // in the set -> black
    if (it == MAXIMUM_ITERATIONS) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    float ratio = float(it) / MAXIMUM_ITERATIONS;
    return vec4(0.0, 0.0, ratio, 1.0);
}

void main() {
    color = make_color();
}
