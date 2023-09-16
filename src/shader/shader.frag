#version 140
#define MAXIMUM_ITERATIONS 100
#define MAXIMUM_DISTANCE_SQUARED 4.0

in vec4 gl_FragCoord;

uniform float zoom;
uniform vec3 julia_c;

out vec4 color;

int julia_iter() {
    float re = (gl_FragCoord.x / 800.0 - 0.5) * zoom;
    float im = (gl_FragCoord.y / 800.0 - 0.5) * zoom;

    float re_c = re;
    float im_c = im;

    float dist2 = re * re + im * im;
    int it = 0;

    while (it < MAXIMUM_ITERATIONS && dist2 < MAXIMUM_DISTANCE_SQUARED) {
        float temp_re = re;

        // iteration step for mandelbrot set
        //re = re * re - im * im + re_c;
        //im = 2.0 * im * temp_re + im_c;

        re = re * re - im * im + julia_c.x;
        im = 2.0 * im * temp_re + julia_c.y;

        dist2 = re * re + im * im;
        it++;
    }

    return it;
}

void main() {
    int it = julia_iter();
    float it_ratio = float(it) / MAXIMUM_ITERATIONS;

    color = vec4(0.0, it_ratio, 0.0, 1.0);
}
