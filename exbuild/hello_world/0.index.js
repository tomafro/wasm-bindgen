(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./hello_world.js":
/*!************************!*\
  !*** ./hello_world.js ***!
  \************************/
/*! exports provided: __wbg_alert_9ea5a791b0d4c7a3, greet, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_9ea5a791b0d4c7a3\", function() { return __wbg_alert_9ea5a791b0d4c7a3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _hello_world_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./hello_world_bg */ \"./hello_world_bg.wasm\");\n/* tslint:disable */\n\n\nconst TextDecoder = typeof self === 'object' && self.TextDecoder\n    ? self.TextDecoder\n    : __webpack_require__(/*! util */ \"../../node_modules/util/util.js\").TextDecoder;\n\nlet cachedDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _hello_world_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_hello_world_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbg_alert_9ea5a791b0d4c7a3(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    alert(varg0);\n}\n\nconst TextEncoder = typeof self === 'object' && self.TextEncoder\n    ? self.TextEncoder\n    : __webpack_require__(/*! util */ \"../../node_modules/util/util.js\").TextEncoder;\n\nlet cachedEncoder = new TextEncoder('utf-8');\n\nfunction passStringToWasm(arg) {\n\n    const buf = cachedEncoder.encode(arg);\n    const ptr = _hello_world_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](buf.length);\n    getUint8Memory().set(buf, ptr);\n    return [ptr, buf.length];\n}\n/**\n* @param {string} arg0\n* @returns {void}\n*/\nfunction greet(arg0) {\n    const [ptr0, len0] = passStringToWasm(arg0);\n    try {\n        return _hello_world_bg__WEBPACK_IMPORTED_MODULE_0__[\"greet\"](ptr0, len0);\n\n    } finally {\n        _hello_world_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr0, len0 * 1);\n\n    }\n\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///./hello_world.js?");

/***/ }),

/***/ "./hello_world_bg.wasm":
/*!*****************************!*\
  !*** ./hello_world_bg.wasm ***!
  \*****************************/
/*! exports provided: memory, __indirect_function_table, __heap_base, __data_end, greet, __rustc_debug_gdb_scripts_section__, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./hello_world */ \"./hello_world.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./hello_world_bg.wasm?");

/***/ })

}]);