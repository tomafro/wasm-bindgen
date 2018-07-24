export function one_two_generator() {
  function* generator() {
    yield 1;
    yield 2;
  }
  return generator();
}

export function dummy_generator() {
  function* generator() {
    const reply = yield '2 * 2';
    return reply === 4;
  }
  return generator();
}

export function broken_generator() {
  function* brokenGenerator() {
    throw new Error('Something went wrong');
    yield 1;
  }
  return brokenGenerator();
}
