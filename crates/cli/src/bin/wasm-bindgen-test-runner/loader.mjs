// originally from https://nodejs.org/api/esm.html
import path from 'path';
import process from 'process';
import Module from 'module';
import fs from 'fs';

const builtins = Module.builtinModules;

const baseURL = new URL('file://');
baseURL.pathname = `${process.cwd()}/`;

const wbgURL = new URL('file://');
wbgURL.pathname = `${process.env.WBG_CRATE_BASE}/`;

export function resolve(specifier, parentModuleURL = baseURL, defaultResolve) {
  if (builtins.includes(specifier)) {
    return {
      url: specifier,
      format: 'builtin'
    };
  }

  if (/^\.{0,2}[/]/.test(specifier) !== true && !specifier.startsWith('file:')) {
    // For node_modules support:
    return defaultResolve(specifier, parentModuleURL);
  }

  let resolved = new URL(specifier, parentModuleURL);

  // Handle a lack of extension by automatically inferring js/mjs
  if (path.extname(resolved.pathname) === '') {
    if (fs.existsSync(resolved.pathname + '.js'))
      resolved.pathname += '.js';
    else if (fs.existsSync(resolved.pathname + '.mjs'))
      resolved.pathname += '.mjs';
  }

  let format = path.extname(resolved.pathname) === '.mjs' ? 'esm' : 'cjs';

  if (!fs.existsSync(resolved.pathname)) {
    resolved = new URL(specifier, baseURL);
    format = 'esm';
  }

  // Make sure this file actually exists.
  if (!fs.existsSync(resolved.pathname))
    throw new Error(`file ${resolved.pathname} does not exist`);


  return {
    url: resolved.href,
    format,
  };
}
