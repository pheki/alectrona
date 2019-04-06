const __exports = {};
import { show_image } from './snippets/alectrona-web-d188a0075a21ad3d/static/dom_manipulator.js';
import { set_logo_list } from './snippets/alectrona-web-d188a0075a21ad3d/static/dom_manipulator.js';
import { reset_logo_list } from './snippets/alectrona-web-d188a0075a21ad3d/static/dom_manipulator.js';
import { enable_bin_input } from './snippets/alectrona-web-d188a0075a21ad3d/static/dom_manipulator.js';
import { enable_replace } from './snippets/alectrona-web-d188a0075a21ad3d/static/dom_manipulator.js';

let wasm;

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function __wbg_showimage_f6e4f8091b281bb0(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getArrayU8FromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = arg4 == 0 ? undefined : getStringFromWasm(arg4, arg5);
    show_image(varg0, varg2, varg4);
}

__exports.__wbg_showimage_f6e4f8091b281bb0 = __wbg_showimage_f6e4f8091b281bb0;

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function getArrayJsValueFromWasm(ptr, len) {
    const mem = getUint32Memory();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function __wbg_setlogolist_a4587e8415eff306(arg0, arg1) {
    let varg0 = getArrayJsValueFromWasm(arg0, arg1);

    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 4);

    set_logo_list(varg0);
}

__exports.__wbg_setlogolist_a4587e8415eff306 = __wbg_setlogolist_a4587e8415eff306;

function __wbg_resetlogolist_926ff71c621ab848() {
    reset_logo_list();
}

__exports.__wbg_resetlogolist_926ff71c621ab848 = __wbg_resetlogolist_926ff71c621ab848;

function __wbg_enablebininput_276a9c36d299677d() {
    enable_bin_input();
}

__exports.__wbg_enablebininput_276a9c36d299677d = __wbg_enablebininput_276a9c36d299677d;

function __wbg_enablereplace_ec7be9dec7f71c9c() {
    enable_replace();
}

__exports.__wbg_enablereplace_ec7be9dec7f71c9c = __wbg_enablereplace_ec7be9dec7f71c9c;
/**
* Initializes devices using POSSIBLE_DEVICES, creating an <option> element on <select id=\"select-device\"> for each device.
* @returns {void}
*/
export function init_devices() {
    return wasm.init_devices();
}

__exports.init_devices = init_devices;

let cachedTextEncoder = new TextEncoder('utf-8');

let WASM_VECTOR_LEN = 0;

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {

        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let writeOffset = 0;
        while (true) {
            const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
            const { read, written } = cachedTextEncoder.encodeInto(arg, view);
            arg = arg.substring(read);
            writeOffset += written;
            if (arg.length === 0) {
                break;
            }
            ptr = wasm.__wbindgen_realloc(ptr, size, size * 2);
            size *= 2;
        }
        WASM_VECTOR_LEN = writeOffset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    };
}
/**
* Saves selected device and enables the binary input field.
* @param {string} codename
* @returns {void}
*/
export function handle_device(codename) {
    const ptr0 = passStringToWasm(codename);
    const len0 = WASM_VECTOR_LEN;
    try {
        return wasm.handle_device(ptr0, len0);

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

__exports.handle_device = handle_device;

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}
/**
* Parses the logo.bin file, saving it on SELECTED_LOGO_BIN and creating the list of logo_ids to select.
* @param {Uint8Array} buffer
* @returns {string}
*/
export function handle_file(buffer) {
    const ptr0 = passArray8ToWasm(buffer);
    const len0 = WASM_VECTOR_LEN;
    const retptr = globalArgumentPtr();
    try {
        wasm.handle_file(retptr, ptr0, len0);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;


    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

__exports.handle_file = handle_file;

/**
* Extracts the image with the id selected and shows it on the <img> element.
* @param {string} logo_id
* @returns {void}
*/
export function handle_logo_id(logo_id) {
    const ptr0 = passStringToWasm(logo_id);
    const len0 = WASM_VECTOR_LEN;
    return wasm.handle_logo_id(ptr0, len0);
}

__exports.handle_logo_id = handle_logo_id;

/**
* Replaces image with the selected logo_id in the logo.bin file.
* @param {Uint8Array} buffer
* @param {string} filename
* @returns {void}
*/
export function handle_image(buffer, filename) {
    const ptr0 = passArray8ToWasm(buffer);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm(filename);
    const len1 = WASM_VECTOR_LEN;
    try {
        return wasm.handle_image(ptr0, len0, ptr1, len1);

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

__exports.handle_image = handle_image;

/**
* Exports the new logo.bin file for download.
* @returns {Uint8Array}
*/
export function export_logo_bin() {
    const retptr = globalArgumentPtr();
    wasm.export_logo_bin(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getArrayU8FromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

}

__exports.export_logo_bin = export_logo_bin;

function __wbg_error_4bb6c2a97407129a(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);

    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);

    console.error(varg0);
}

__exports.__wbg_error_4bb6c2a97407129a = __wbg_error_4bb6c2a97407129a;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function __wbg_new_59cb74e423758ede() {
    return addHeapObject(new Error());
}

__exports.__wbg_new_59cb74e423758ede = __wbg_new_59cb74e423758ede;

function __wbg_stack_558ba5917b466edd(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).stack);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

__exports.__wbg_stack_558ba5917b466edd = __wbg_stack_558ba5917b466edd;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function __widl_f_get_element_by_id_Document(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = getObject(arg0).getElementById(varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_get_element_by_id_Document = __widl_f_get_element_by_id_Document;

function __widl_instanceof_Element(idx) { return getObject(idx) instanceof Element ? 1 : 0; }

__exports.__widl_instanceof_Element = __widl_instanceof_Element;

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

function __widl_f_remove_attribute_Element(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).removeAttribute(varg1);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_remove_attribute_Element = __widl_f_remove_attribute_Element;

function __widl_f_set_attribute_Element(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setAttribute(varg1, varg3);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_set_attribute_Element = __widl_f_set_attribute_Element;

function __widl_f_append_child_Node(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).appendChild(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_append_child_Node = __widl_f_append_child_Node;

function __widl_f_clone_node_with_deep_Node(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).cloneNode(arg1 !== 0));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_clone_node_with_deep_Node = __widl_f_clone_node_with_deep_Node;

function __widl_f_child_nodes_Node(arg0) {
    return addHeapObject(getObject(arg0).childNodes);
}

__exports.__widl_f_child_nodes_Node = __widl_f_child_nodes_Node;

function __widl_f_set_text_content_Node(arg0, arg1, arg2) {
    let varg1 = arg1 == 0 ? undefined : getStringFromWasm(arg1, arg2);
    getObject(arg0).textContent = varg1;
}

__exports.__widl_f_set_text_content_Node = __widl_f_set_text_content_Node;

function __widl_f_get_NodeList(arg0, arg1) {

    const val = getObject(arg0)[arg1];
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_get_NodeList = __widl_f_get_NodeList;

function __widl_f_length_NodeList(arg0) {
    return getObject(arg0).length;
}

__exports.__widl_f_length_NodeList = __widl_f_length_NodeList;

function __widl_instanceof_Window(idx) { return getObject(idx) instanceof Window ? 1 : 0; }

__exports.__widl_instanceof_Window = __widl_instanceof_Window;

function __widl_f_alert_with_message_Window(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).alert(varg1);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_alert_with_message_Window = __widl_f_alert_with_message_Window;

function __widl_f_document_Window(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_document_Window = __widl_f_document_Window;

function __wbg_newnoargs_b4526aa2a6db81de(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

__exports.__wbg_newnoargs_b4526aa2a6db81de = __wbg_newnoargs_b4526aa2a6db81de;

function __wbg_call_a7a8823c404228ab(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__wbg_call_a7a8823c404228ab = __wbg_call_a7a8823c404228ab;

function __wbindgen_string_new(p, l) { return addHeapObject(getStringFromWasm(p, l)); }

__exports.__wbindgen_string_new = __wbindgen_string_new;

function __wbindgen_debug_string(i, len_ptr) {
    const debug_str =
    val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}

__exports.__wbindgen_debug_string = __wbindgen_debug_string;

function __wbindgen_rethrow(idx) { throw takeObject(idx); }

__exports.__wbindgen_rethrow = __wbindgen_rethrow;

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

__exports.__wbindgen_throw = __wbindgen_throw;

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './alectrona_web': __exports };
    if (module_or_path instanceof URL || typeof module_or_path === 'string' || module_or_path instanceof Request) {

        const response = fetch(module_or_path);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module_or_path, imports)
        .then(instance => {
            return { instance, module: module_or_path };
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;

