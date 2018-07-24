const key = Symbol();

export function map_with_symbol_key() {
  return { [key]: 42 };
}
export function symbol_key() {
  return key;
}

export class Foo {}
export class Bar {}
