//for browsers that don't support top-level await
(async () => { 

const WIDTH = 500;
const HEIGHT = 500;
const BUFSIZE = WIDTH * HEIGHT * 4;
const ROTRATE = 5; //seconds per rotation

const WASM_URL = "meenle-noonle.wasm";

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
unavailable. no Meenle-Noonle for you! you are missing out! :(";
}

instance.exports.generate_background();
instance.exports.set_mesh(0);
document.addEventListener('keydown', function(event) {
    switch (event.code) {
        case "Digit1":
            instance.exports.set_mesh(0);
            break;
        case "Digit2":
            instance.exports.set_mesh(1);
            break;
        case "Digit3":
            instance.exports.set_mesh(2);
            break;
    }
});


let bufptr = instance.exports.get_buffer();
let buffer = new Uint8ClampedArray(instance.exports.memory.buffer, bufptr, BUFSIZE);
let imgData, imgBmap;
if (!bmapSupport) { console.warn("ImageBitmap not supported. Falling back to ImageData. Your browser is likely old, \
please upgrade."); }

async function pushBuffer() {
    bufptr = instance.exports.get_buffer();
    buffer = new Uint8ClampedArray(instance.exports.memory.buffer, bufptr, BUFSIZE);
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
    instance.exports.render_spin(Date.now() / 1000, ROTRATE);
    pushBuffer();
    window.requestAnimationFrame(onAnimFrame);
}

window.requestAnimationFrame(onAnimFrame);

})()