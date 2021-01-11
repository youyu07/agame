#version 450
layout(location = 0) in vec3 v_pos;
layout(location = 0) out vec4 o_Target;

#define PI 3.1415926538
#define RAYLEIGH 1.0
#define REFRACTIVE_INDEX 1.0003
#define DEPOLARIZATION_FACTOR 1000.0
#define NUM_MOLECULES 2.542e25
#define MIE_V 4.0
#define MIE_COEFFICIENT 0.005
#define MIE_DIRECTIONAL_G 0.8
#define MIE_ZENITH_LENGTH 1.25e3
#define TURBIDITY 2.0
#define SUN_INTENSITY_FACTOR 1000.0
#define SUN_INTENSITY_FALLOFF_STEEPNESS 1.5
#define RAYLEIGH_ZENITH_LENGTH 8.4e3
#define SUN_ANGULAR_DIAMETER_DEGREES 0.0093333
#define PRIMARIES vec3(6.8e-7, 5.5e-7, 4.5e-7)
#define MIE_K_COEFFICIENT vec3(0.686, 0.678, 0.666)

float acos_approx(float v)
{
    float x = abs(v);
    float res = -0.155972 * x + 1.56467;
    res = res * sqrt(1.0 - x);
    return v >= 0.0 ? res : PI - res;
}

vec3 tonemap(vec3 col)
{
    const float A = 2.35;
    const float B = 2.8826666;
    const float C = 789.7459;
    const float D = 0.935;
    vec3 z = pow(col, vec3(A));
    return z / (pow(z, vec3(D)) * B + C);
}

vec3 total_rayleigh(vec3 lambda)
{
    return (8.0 * pow(PI, 3.0) * pow(pow(REFRACTIVE_INDEX, 2.0) - 1.0, 2.0) * (6.0 + 3.0 * DEPOLARIZATION_FACTOR))
            / (3.0 * NUM_MOLECULES * pow(lambda, vec3(4.0)) * (6.0 - 7.0 * DEPOLARIZATION_FACTOR));
}

vec3 total_mie(vec3 lambda, vec3 k, float t)
{
    float c = 0.2 * t * 10e-18;
    return 0.434 * c * PI * pow(vec3(2.0 * PI) / lambda, vec3(MIE_V - 2.0)) * k;
}

float rayleigh_phase(float cos_theta)
{
    return (3.0 / (16.0 * PI)) * (1.0 - pow(cos_theta, 2.0));
}

float henyey_greenstein_phase(float cos_theta, float g)
{
    return (1.0 / (4.0 * PI)) * pow(((1.0 - pow(g, 2.0)) / (1.0 - 2.0 * g * cos_theta + pow(g, 2.0))), 1.5);
}

float sun_intensity(float zenith_angle_cos)
{
    float cutoff_angle = PI / 1.95;
    return SUN_INTENSITY_FACTOR * max(0.0, 1.0 - exp(-((cutoff_angle - acos_approx(zenith_angle_cos)) / SUN_INTENSITY_FALLOFF_STEEPNESS)));
}

vec3 sky(vec3 dir, vec3 sun_position)
{
    vec3 up = vec3(0.0,1.0,0.0);
    float sunfade = 1.0 - (1.0 - exp(clamp(sun_position.y / 450000.0, 0.0, 1.0)));
    float rayleigh_coefficient = RAYLEIGH - (1.0 * (1.0 - sunfade));
    vec3 beta_r = total_rayleigh(PRIMARIES) * rayleigh_coefficient;

    vec3 beta_m = total_mie(PRIMARIES, MIE_K_COEFFICIENT, TURBIDITY) * MIE_COEFFICIENT;
    
    float zenith_angle = acos_approx(max(0.0,dot(up, dir)));
    float denom = cos(zenith_angle) + 0.15 * pow(93.885 - (zenith_angle * 180.0 / PI), -1.253);

    float s_r = RAYLEIGH_ZENITH_LENGTH / denom;
    float s_m = MIE_ZENITH_LENGTH / denom;

    vec3 fex = exp(-(beta_r * s_r + beta_m * s_m));

    vec3 sun_direction = normalize(sun_position);
    float cos_theta = dot(dir, sun_direction);
    vec3 beta_r_theta = beta_r * rayleigh_phase(cos_theta * 0.5 + 0.5);

    vec3 beta_m_theta = beta_m * henyey_greenstein_phase(cos_theta, MIE_DIRECTIONAL_G);
    float sun_e = sun_intensity(dot(sun_direction, up));
    vec3 lin = pow(sun_e * ((beta_r_theta + beta_m_theta) / (beta_r + beta_m)) * (vec3(1.0) - fex), vec3(1.5));
    lin = lin * mix(
        vec3(1.0), 
        pow(sun_e * ((beta_r_theta + beta_m_theta)/(beta_r + beta_m)) * fex, vec3(0.5)), 
        clamp(pow(1.0 - dot(up, sun_direction), 5.0), 0.0, 1.0)
    );

    float sun_angular_diameter_cos = cos(SUN_ANGULAR_DIAMETER_DEGREES);
    float sundisk = smoothstep(sun_angular_diameter_cos , sun_angular_diameter_cos + 0.00002, cos_theta);
    vec3 l0 = 0.1 * fex;
    l0 = l0 + sun_e * 19000.0 * fex * sundisk;

    return lin + l0;
}

vec3 get_ray_dir(vec2 uv, vec3 pos, vec3 look_at_pos)
{
    vec3 forward = normalize(look_at_pos - pos);
    vec3 right = normalize(cross(vec3(0.0,1.0,0.0), forward));
    vec3 up = cross(forward, right);
    return normalize(forward + uv.x * right + uv.y * up);
}

vec4 fs()
{
    vec2 view_size = vec2(1280.0, 720.0);

    vec2 uv = (v_pos.xy - 0.5 * view_size) / view_size.y;
    uv.y = -uv.y;

    vec3 eye_pos = vec3(0.0, 0.0997, 0.2);
    vec3 sun_pos = vec3(0.0, 75.0, -1000.0);
    vec3 dir = get_ray_dir(uv, eye_pos, sun_pos);

    vec3 color = sky(dir, sun_pos);
    color = clamp(color, vec3(0.0), vec3(1024.0));

    return vec4(tonemap(color), 1.0);
}


void main() {
    o_Target = fs();
}