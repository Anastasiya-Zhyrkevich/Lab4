#pragma version(1)
#pragma rs java_package_name(com.example.android.rs.hellocompute)

rs_allocation gIn;
rs_allocation gOut;
rs_script gScript;
float brightness;

void root(const uchar4 *v_in, uchar4 *v_out, const void *usrData, uint32_t x, uint32_t y) {
    float4 f4 = rsUnpackColor8888(*v_in);
    f4.r += brightness;

    if(f4.r > 1){
    	f4.r = 1;
    }

    f4.g += brightness;
    if(f4.g > 1){
    	f4.g = 1;
    }

    f4.b += brightness;
    if(f4.b > 1){
    	f4.b = 1;
    }

    float3 output = {f4.r, f4.g, f4.b};

    *v_out = rsPackColorTo8888(output);
}

void filter() {
    rsForEach(gScript, gIn, gOut, 0);

}

