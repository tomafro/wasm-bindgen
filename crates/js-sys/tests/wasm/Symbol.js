function assert_eq(a, b) {
  if (a === b)
    return;
  throw new Error(`${a} !== ${b}`)
}

function assert(a) {
  if (a)
    return;
  throw new Error(`assert(false) called`);
}

function assert_throws(f) {
  try {
    f();
  } catch(e) {
    return
  }

  throw new Error('function did not throw');
}

function arrays_equal(a, b) {
  assert_eq(a.length, b.length);
  for (let i = 0; i < a.length; i++) {
    const ai = a[i];
    const bi = b[i];
    if (ai instanceof Array) {
      arrays_equal(ai, bi);
    } else {
      assert_eq(ai, bi);
    }
  }
}

export function test_has_instance(sym) {
  class Array1 {
    static [sym](instance) {
      return Array.isArray(instance);
    }
  }

  assert_eq(typeof sym, "symbol");
  assert([] instanceof Array1);
}

export function test_is_concat_spreadable(sym) {
  const alpha = ['a', 'b', 'c'];
  const numeric = [1, 2, 3];
  let alphaNumeric = alpha.concat(numeric);

  arrays_equal(alphaNumeric, ["a", "b", "c", 1, 2, 3]);

  numeric[sym] = false;
  alphaNumeric = alpha.concat(numeric);

  arrays_equal(alphaNumeric, ["a", "b", "c", [1, 2, 3]]);
}

export function test_iterator(sym) {
  const iterable1 = new Object();

  iterable1[sym] = function* () {
    yield 1;
    yield 2;
    yield 3;
  };

  arrays_equal([...iterable1], [1, 2, 3]);
}

export function test_match(sym) {
  const regexp1 = /foo/;
  assert_throws(() => '/foo/'.startsWith(regexp1));

  regexp1[sym] = false;

  assert('/foo/'.startsWith(regexp1));

  assert_eq('/baz/'.endsWith(regexp1), false);
}

export function test_replace(sym) {
  class Replace1 {
    constructor(value) {
      this.value = value;
    }
    [sym](string) {
      return `s/${string}/${this.value}/g`;
    }
  }

  assert_eq('foo'.replace(new Replace1('bar')), 's/foo/bar/g');
}

export function test_search(sym) {
  class Search1 {
    constructor(value) {
      this.value = value;
    }

    [sym](string) {
      return string.indexOf(this.value);
    }
  }

  assert_eq('foobar'.search(new Search1('bar')), 3);
}

export function test_species(sym) {
  class Array1 extends Array {
    static get [sym]() { return Array; }
  }

  const a = new Array1(1, 2, 3);
  const mapped = a.map(x => x * x);

  assert_eq(mapped instanceof Array1, false);

  assert(mapped instanceof Array);
}

export function test_split(sym) {
  class Split1 {
    constructor(value) {
      this.value = value;
    }

    [sym](string) {
      var index = string.indexOf(this.value);
      return this.value + string.substr(0, index) + "/"
        + string.substr(index + this.value.length);
    }
  }

  assert_eq('foobar'.split(new Split1('foo')), 'foo/bar');
}

export function test_to_primitive(sym) {
  const object1 = {
    [sym](hint) {
      if (hint == 'number') {
        return 42;
      }
      return null;
    }
  };

  assert_eq(+object1, 42);
}

export function test_to_string_tag(sym) {
  class ValidatorClass {
    get [sym]() {
      return 'Validator';
    }
  }

  assert_eq(Object.prototype.toString.call(new ValidatorClass()), '[object Validator]');
}
