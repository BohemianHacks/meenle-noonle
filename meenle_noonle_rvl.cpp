// Link with the Revolution SDK and add it to your include path
// Also link with meenle-noonle

#include <revolution.h> // Wii libraries
#include <math.h> // Trig functions for 3D
#include <string>

// const u32 RUST_HEAP_SIZE = 0x400000; // 4 MiB
const u32 RUST_HEAP_SIZE = 0x001000; // 4 KiB
const u32 MNFB_WIDTH = 500; // MNFB = Meenle-Noonle Frame Buffer
const f64 TAU =  3.14159265358979323846 * 2;
const f64 ROTRATE = 5; // seconds per rotation

const GXColor black = {0, 0, 0, 255};
const GXColor white = {255, 255, 255, 255};

struct ColorRGBA { u8 r; u8 g; u8 b; u8 a; };
struct ColorYUV { u8 Y; u8 U; u8 V; };
typedef u32 ColorYUYV; // 0xYYUUYYVV; YUYV color format encodes two pixels in four bytes

// Functions from rust
extern "C" { 
    void generate_background();
    void render(f64 scalar, f64 x_angle, f64 y_angle, f64 z_angle);
    ColorRGBA* get_buffer();
    void drawMNFBIntoXFB(void* mnfb, void* xfb, u32 fb_width);
}

void setupRustHeap() {
    OSSetMEM1ArenaLo(OSInitAlloc(OSGetMEM1ArenaLo(), OSGetMEM1ArenaHi(), 1));
    void* rust_heap = OSAllocFromMEM1ArenaLo(RUST_HEAP_SIZE, 32);
    OSHeapHandle rust_heap_handle = OSCreateHeap(rust_heap, OSGetMEM1ArenaLo()); // should be zero
    OSSetCurrentHeap(rust_heap_handle);
}

// VI = Video Interface
void setupVI(const GXRenderModeObj &rmode, const u32 xfbSize, void* &xfb1, void* &xfb2) {
    VIInit();
    xfb1 = OSAllocFromMEM1ArenaLo(xfbSize, 32);
    xfb2 = OSAllocFromMEM1ArenaLo(xfbSize, 32);
    VIConfigure(&rmode);
    VISetNextFrameBuffer(xfb1);
    VISetBlack(FALSE);
    VIFlush();
    VIWaitForRetrace();
}

ColorYUV RGBAtoYUV(const ColorRGBA color) {
    u8 Ychar, Uchar, Vchar;
    f32 Y =  0.257 * color.r + 0.504 * color.g + 0.098 * color.b +  16;
    f32 U = -0.148 * color.r - 0.291 * color.g + 0.439 * color.b + 128;
    f32 V =  0.439 * color.r - 0.368 * color.g - 0.071 * color.b + 128;
    OSf32tou8(&Y, &Ychar);
    OSf32tou8(&U, &Uchar);
    OSf32tou8(&V, &Vchar);
    ColorYUV ret = {Ychar, Uchar, Vchar};
    return ret;
}

// TODO: for best color output, calculate from three colors: i-1, i, i+1
extern "C" ColorYUYV RGBAtoYUYV(const ColorRGBA color1, const ColorRGBA color2) {
    ColorYUV YUV1 = RGBAtoYUV(color1);
    ColorYUV YUV2 = RGBAtoYUV(color2);
    u8 U = 0.5 * YUV1.U + 0.5 * YUV2.U;
    u8 V = 0.5 * YUV1.V + 0.5 * YUV2.V;
    return (YUV1.Y << 24) + (U << 16) + (YUV2.Y << 8) + V;
}

void fillFBWithColor(void* xfb, u32 &xfbSize, ColorRGBA RGBA) {
    u32 color = RGBAtoYUYV(RGBA, RGBA);
    u32 pxl = 0;
    u8* fb = (u8*) xfb;
    for (u8* ptr = fb; ptr < fb + xfbSize; ptr += VI_DISPLAY_PIX_SZ * 2) {
        *(u32*)ptr = color;
        pxl++;
    }
}

void main() {
    OSInitFastCast();
    setupRustHeap();

    const GXRenderModeObj rmode = GXNtsc480Prog;
    const u32 xfbSize = VIPadFrameBufferWidth(rmode.fbWidth) * rmode.xfbHeight * VI_DISPLAY_PIX_SZ;
    void *xfb1, *xfb2;
    setupVI(rmode, xfbSize, xfb1, xfb2);

    generate_background();
    ColorRGBA* mnfb = get_buffer(); // meenle-noonle frame buffer
    render(0, 0, 0, 0);

    u32 frameNum;
    while (1) {
        frameNum = VIGetRetraceCount();
        void* xfbNext = (VIGetCurrentFrameBuffer() == xfb2? xfb1: xfb2);
        VISetNextFrameBuffer(xfbNext);

        render(2, TAU / 2, ((frameNum % 3600 ) / 60.0 * TAU / ROTRATE), 0);
        drawMNFBIntoXFB(mnfb, xfbNext, rmode.fbWidth);

        VIFlush();
        VIWaitForRetrace();
    }

    PPCHalt(); // unreachable
}
