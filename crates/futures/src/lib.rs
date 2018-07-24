#![feature(use_extern_macros)]

extern crate futures;
extern crate wasm_bindgen;
extern crate js_sys;

use std::sync::Arc;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut, Cell};

use futures::executor::{self, Spawn, Notify};
use futures::prelude::*;
use futures::task::{self, Task};
use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type Promise;

    #[wasm_bindgen(constructor)]
    pub fn new(cb: &mut FnMut(Function, Function)) -> Promise;
    #[wasm_bindgen(static_method_of = Promise)]
    pub fn all(obj: JsValue) -> Promise;
    #[wasm_bindgen(static_method_of = Promise)]
    pub fn race(obj: JsValue) -> Promise;
    #[wasm_bindgen(static_method_of = Promise)]
    pub fn reject(obj: JsValue) -> Promise;
    #[wasm_bindgen(static_method_of = Promise)]
    pub fn resolve(obj: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn catch(this: &Promise, cb: &Closure<FnMut(JsValue)>) -> Promise;
    #[wasm_bindgen(method)]
    pub fn then(this: &Promise, cb: &Closure<FnMut(JsValue)>) -> Promise;
    #[wasm_bindgen(method, js_name = then)]
    pub fn then2(this: &Promise,
                 resolve: &Closure<FnMut(JsValue)>,
                 reject: &Closure<FnMut(JsValue)>) -> Promise;
    #[wasm_bindgen(method)]
    pub fn finally(this: &Promise, cb: &Closure<FnMut()>) -> Promise;
}

pub struct JsFuture {
    promise: Promise,
    state: Rc<RefCell<State>>,
    registered: bool,
}

#[derive(Default)]
struct State {
    result: Option<Result<JsValue, JsValue>>,
    task: Option<Task>,
    callbacks: Option<(Closure<FnMut(JsValue)>, Closure<FnMut(JsValue)>)>,
}

impl JsFuture {
    fn register<'a>(&'a mut self) -> RefMut<'a, State> {
        if self.registered {
            return self.state.borrow_mut();
        }
        self.registered = true;
        let s = Rc::downgrade(&self.state);
        let cb1 = Closure::wrap(Box::new(move |val| finish(&s, Ok(val))) as Box<_>);
        let s = Rc::downgrade(&self.state);
        let cb2 = Closure::wrap(Box::new(move |val| finish(&s, Err(val))) as Box<_>);
        self.promise.then2(&cb1, &cb2);
        self.registered = true;
        let mut state = self.state.borrow_mut();
        state.callbacks = Some((cb1, cb2));
        return state
    }
}

impl Future for JsFuture {
    type Item = JsValue;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<JsValue, JsValue> {
        let mut state = self.register();
        match state.result.take() {
            Some(Ok(val)) => return Ok(val.into()),
            Some(Err(val)) => return Err(val),
            None => {}
        }
        state.task = Some(task::current());
        Ok(Async::NotReady)
    }
}

fn finish(state: &Weak<RefCell<State>>, result: Result<JsValue, JsValue>) {
    let task = {
        let state = match state.upgrade() {
            Some(s) => s,
            None => return,
        };
        let mut s = state.borrow_mut();
        assert!(s.result.is_none());
        s.result = Some(result);
        assert!(s.callbacks.take().is_some());
        s.task.take()
    };
    if let Some(task) = task {
        task.notify();
    }
}

impl From<Promise> for JsFuture {
    fn from(js: Promise) -> JsFuture {
        JsFuture {
            promise: js,
            registered: false,
            state: Default::default(),
        }
    }
}

pub fn rust2js<F>(future: F) -> Promise
    where F: Future<Item = JsValue, Error = JsValue> + 'static,
{
    _rust2js(Box::new(future))
}

fn _rust2js(future: Box<Future<Item = JsValue, Error = JsValue>>) -> Promise {
    let mut future = Some(executor::spawn(future));
    return Promise::new(&mut |resolve, reject| {
        Package::poll(&Arc::new(Package {
            spawn: RefCell::new(future.take().unwrap()),
            resolve,
            reject,
            notified: Cell::new(State::Notified),
            me: RefCell::new(None),
        }));
    });

    struct Package {
        spawn: RefCell<Spawn<Box<Future<Item = JsValue, Error = JsValue>>>>,
        notified: Cell<State>,
        me: RefCell<Option<Arc<Package>>>,
        resolve: Function,
        reject: Function,
    }

    enum State {
        Polling,
        Notified,
        Waiting,
    }

    // No shared memory right now, wasm is single threaded, no need to worry
    // about this!
    unsafe impl Send for Package {}
    unsafe impl Sync for Package {}

    impl Package {
        fn poll(me: &Arc<Package>) {
            while let State::Notified = me.notified.replace(State::Polling) {
                let (val, f) = match me.spawn.borrow_mut().poll_future_notify(me, 0) {
                    // If the future is ready, immediately call the
                    // resolve/reject callback and then return as we're done.
                    Ok(Async::Ready(value)) => (value, &me.resolve),
                    Err(value) => (value, &me.reject),

                    // Otherwise keep going in our loop, if we weren't notified
                    // we'll break out and start waiting.
                    Ok(Async::NotReady) => continue,
                };
                let array = Array::new();
                array.push(val);
                drop(f.apply(&JsValue::undefined(), &array));
                return
            }

            // At this point we know that we're in the "polling" state and
            // haven't been notified. The only way currently for a notification
            // to come in through JS is through some other callback on the
            // event loop. Consequently we simply flag ourselves as waiting and
            // wait for such a callback to fire.
            //
            // When the callback fires it'll notify our task, which below will
            // resume the polling process.
            me.notified.set(State::Waiting);
            *me.me.borrow_mut() = Some(me.clone());
        }
    }

    impl Notify for Package {
        fn notify(&self, _id: usize) {
            match self.notified.replace(State::Notified) {
                // we need to schedule polling to resume, so we do so
                // immediately for now
                State::Waiting => {}
                // our notification is queued up for later
                State::Notified | State::Polling => return,
            }
            if let Some(me) = self.me.borrow_mut().take() {
                Package::poll(&me);
            }
        }
    }
}
