export function get_function_to_bind() {
  return function() {
    return (this || {}).x || 1;
  }
}

export function get_value_to_bind_to() {
  return { x: 2 };
}

export function call_function(f) {
  return f();
}
