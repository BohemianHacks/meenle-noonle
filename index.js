//for browsers that don't support top-level await
(async () => { 

const WIDTH = 500;
const HEIGHT = 500;
const BUFSIZE = WIDTH * HEIGHT * 4;
const TAU = Math.PI * 2;
const ROTRATE = 5; //seconds per rotation

const WASM_URL = "target/wasm32-unknown-unknown/release/meenle_noonle.wasm";

let bmapSupport = typeof createImageBitmap !== "undefined";
let viewport = document.getElementById("viewport");
let ctx = viewport.getContext(bmapSupport ? "bitmaprenderer" : "2d");

// load wasm module
if (WebAssembly.instantiateStreaming) {
    var instance = (await WebAssembly.instantiateStreaming(fetch(WASM_URL))).instance;
} else if (WebAssembly.instantiate) {
    console.warn("WebAssembly.instantiateStreaming not supported. Falling back to WebAssembly.instantiate. \
Your browser is likely old, please upgrade.")
    let wasm = await fetch(WASM_URL);
    let arrBuff = await wasm.arrayBuffer();
    var instance = (await WebAssembly.instantiate(arrBuff)).instance;
} else {
    console.warn("Wasm unsupported! Upgrade your browser.");
    document.getElementById("title").textContent = "your browser is probably old, because wasm is \
unavailable. no Meenlo-Noonle for you! you are missing out!";
}

instance.exports.generate_background();
instance.exports.render(0, 0, 0 ,0);

let bufptr = instance.exports.get_buffer();
let buffer = new Uint8ClampedArray(instance.exports.memory.buffer, bufptr, BUFSIZE);
let imgData, imgBmap;
if (!bmapSupport) { console.warn("ImageBitmap not supported. Falling back to ImageData. Your browser is likely old, \
please upgrade."); }

async function pushBuffer() {
    imgData = new ImageData(buffer, WIDTH, HEIGHT);
    if (bmapSupport) {
        imgBmap = await createImageBitmap(imgData);
        ctx.transferFromImageBitmap(imgBmap);
    } else {
        ctx.putImageData(imgData, 0, 0);    
    }
}

// runs every frame, rotates model @ ROTRATE seconds per rotation
function onAnimFrame() {
    instance.exports.render(2, TAU / 2, (Date.now() / 1000 * TAU / ROTRATE) % TAU, 0);
    pushBuffer();
    window.requestAnimationFrame(onAnimFrame);
}

window.requestAnimationFrame(onAnimFrame);

})()