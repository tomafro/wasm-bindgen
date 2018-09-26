(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./char.js":
/*!*****************!*\
  !*** ./char.js ***!
  \*****************/
/*! exports provided: __wbg_log_ba93d378c95b0bd2, Counter, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_ba93d378c95b0bd2\", function() { return __wbg_log_ba93d378c95b0bd2; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Counter\", function() { return Counter; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _char_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./char_bg */ \"./char_bg.wasm\");\n/* tslint:disable */\n\n\nconst __wbg_log_ba93d378c95b0bd2_target = console.log;\n\nconst TextDecoder = typeof self === 'object' && self.TextDecoder\n    ? self.TextDecoder\n    : __webpack_require__(/*! util */ \"../../node_modules/util/util.js\").TextDecoder;\n\nlet cachedDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _char_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_char_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbg_log_ba93d378c95b0bd2(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    __wbg_log_ba93d378c95b0bd2_target(varg0);\n}\n\nfunction freeCounter(ptr) {\n\n    _char_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_counter_free\"](ptr);\n}\n/**\n*/\nclass Counter {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Counter.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n        freeCounter(ptr);\n    }\n    /**\n    * @returns {Counter}\n    */\n    static default() {\n        return Counter.__wrap(_char_bg__WEBPACK_IMPORTED_MODULE_0__[\"counter_default\"]());\n    }\n    /**\n    * @param {string} arg0\n    * @param {number} arg1\n    * @returns {Counter}\n    */\n    static new(arg0, arg1) {\n        return Counter.__wrap(_char_bg__WEBPACK_IMPORTED_MODULE_0__[\"counter_new\"](arg0.codePointAt(0), arg1));\n    }\n    /**\n    * @returns {string}\n    */\n    key() {\n        return String.fromCodePoint(_char_bg__WEBPACK_IMPORTED_MODULE_0__[\"counter_key\"](this.ptr));\n    }\n    /**\n    * @returns {number}\n    */\n    count() {\n        return _char_bg__WEBPACK_IMPORTED_MODULE_0__[\"counter_count\"](this.ptr);\n    }\n    /**\n    * @returns {void}\n    */\n    increment() {\n        return _char_bg__WEBPACK_IMPORTED_MODULE_0__[\"counter_increment\"](this.ptr);\n    }\n    /**\n    * @param {string} arg0\n    * @returns {void}\n    */\n    update_key(arg0) {\n        return _char_bg__WEBPACK_IMPORTED_MODULE_0__[\"counter_update_key\"](this.ptr, arg0.codePointAt(0));\n    }\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///./char.js?");

/***/ }),

/***/ "./char_bg.wasm":
/*!**********************!*\
  !*** ./char_bg.wasm ***!
  \**********************/
/*! exports provided: memory, __indirect_function_table, __heap_base, __data_end, __rustc_debug_gdb_scripts_section__, __wbg_counter_free, counter_default, counter_new, counter_key, counter_count, counter_increment, counter_update_key */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./char */ \"./char.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./char_bg.wasm?");

/***/ })

}]);