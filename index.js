//for browsers that don't support top-level await
(async () => { 

const WIDTH = 500;
const HEIGHT = 500;
const BUFSIZE = WIDTH * HEIGHT * 4;
const TAU = Math.PI * 2
//seconds per rotation
const ROTRATE = 5

const WASM_URL = "target/wasm32-unknown-unknown/debug/meenle_noonle.wasm";


let viewport = document.getElementById("viewport");
let ctx = viewport.getContext("2d")

// load wasm module
if (WebAssembly.instantiateStreaming) {
    var instance = (await WebAssembly.instantiateStreaming(fetch(WASM_URL))).instance;
} else if (WebAssembly.instantiate) {
    console.warn("WebAssembly.instantiateStreaming not supported. Falling back to WebAssembly.instantiate. \
Your browser is likely old, please upgrade.")
    let wasm = await fetch(WASM_URL);
    let arrBuff = await wasm.arrayBuffer();
    var instance= (await WebAssembly.instantiate(arrBuff)).instance;
} else {
    console.warn("Wasm unsupported! Upgrade your browser.")
    document.getElementById("title").textContent = "your browser is probably old, because wasm is \
unavailable. no Meenlo-Noonle for you! you are missing out!"
}

instance.exports.generate_background()
instance.exports.render(0, 0, 0 ,0)

let bufptr = instance.exports.get_buffer();
let buffer = new Uint8ClampedArray(instance.exports.memory.buffer, bufptr, BUFSIZE)
let imgData, imgBmap;
let bmapSupport = typeof createImageBitmap !== "undefined";
if (!bmapSupport) {
    console.warn("ImageBitmap not supported. Falling back to ImageData. Your browser is likely old, please upgrade.")
}

async function pushBuffer() {
    imgData = new ImageData(buffer, WIDTH, HEIGHT);
    if (bmapSupport) {
        imgBmap = await createImageBitmap(imgData);
        ctx.drawImage(imgBmap, 0, 0);
    } else {
        ctx.putImageData(imgData, 0, 0);    
    }
}

// runs every frame, rotates model @ ROTRATE seconds per rotation
function onAnimFrame(timestamp) {
    instance.exports.render(2, 0, (timestamp / 1000 * TAU / ROTRATE) % TAU, TAU / 2)
    pushBuffer()
    window.requestAnimationFrame(onAnimFrame)
}

window.requestAnimationFrame(onAnimFrame)

})()