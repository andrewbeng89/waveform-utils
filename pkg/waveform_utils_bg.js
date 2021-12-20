import * as wasm from './waveform_utils_bg.wasm';

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getFloat32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
* @param {Float32Array} numbers
* @param {number} size
* @returns {Float32Array}
*/
export function reduce_waveform_data(numbers, size) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        var ptr0 = passArrayF32ToWasm0(numbers, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.reduce_waveform_data(retptr, ptr0, len0, size);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayF32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @param {Float32Array} numbers
* @param {number} size
* @returns {number}
*/
export function reduce_waveform_data_ptr(numbers, size) {
    var ptr0 = passArrayF32ToWasm0(numbers, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.reduce_waveform_data_ptr(ptr0, len0, size);
    return ret;
}

