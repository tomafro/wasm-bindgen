export function proxy_target() {
  return { a: 100 };
}

export function proxy_handler() {
  return {
    get: function(obj, prop) {
      return prop in obj ? obj[prop] : 37;
    }
  };
}
