use pulse::mainloop::standard::Mainloop;
use pulse::context::{introspect::Introspector, Context, FlagSet as ContextFlagSet};
use pulse::proplist::Proplist;
use pulse::mainloop::standard::IterateResult;
use pulse::operation;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

struct IntrospectWrapper {
    introspect: Rc<RefCell<Introspector>>,
    ctx_ref: Rc<RefCell<Context>>
}

impl IntrospectWrapper {
    pub fn new(ctx: Rc<RefCell<Context>>) -> IntrospectWrapper {
        IntrospectWrapper {
            introspect: Rc::new(RefCell::new(ctx.borrow().introspect()))
            ctx_ref = ctx.take()
        }
    }
}

pub struct PaApp {
    mainloop: Rc<RefCell<Mainloop>>,
    context: Rc<RefCell<Context>>,
    pub introspect: 
}

fn pa_wait_for<F>(mainloop: &mut Mainloop, f: F) -> Result<(), &'static str>
where F: Fn() -> Option<Result<(), &'static str>> {
    loop {
        match mainloop.iterate(true) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                return Err("Iterate state was not success, quitting...");
            },
            IterateResult::Success(_) => {},
        }
        if let Some(result) = f() {
            return result;
        }
    }
}

fn pa_wait_result<T: ?Sized>(mainloop: &mut Mainloop, result: operation::Operation<T>) -> Result<(), &'static str> {
    pa_wait_for(mainloop, || {
        match result.get_state() {
            operation::State::Done => Some(Ok(())),
            operation::State::Cancelled => Some(Err("Operation canceled")),
            operation::State::Running => None,
        }
    })
}

impl PaApp {
    pub fn new(name: &str) -> PaApp {
        let mut proplist = Proplist::new().unwrap();
        proplist.set_str(
            pulse::proplist::properties::APPLICATION_NAME, name
        ).unwrap();

        let mut mainloop = Rc::new(RefCell::new(Mainloop::new().expect("Failed to create mainloop")));
    
        println!("refcount1: {}", Rc::strong_count(&mainloop));
        let mut context = Rc::new(RefCell::new(
            Context::new_with_proplist(mainloop.borrow().deref(), name /*+ "Context"*/, &proplist)
            .expect("Failed to create new context")
        ));
        println!("refcount3: {}", Rc::strong_count(&mainloop));
    
        println!("crefcount1: {}", Rc::strong_count(&context));
        context.borrow_mut().connect(None, ContextFlagSet::NOFLAGS, None)
            .expect("Failed to connect context");
    
        pa_wait_for(&mut mainloop.borrow_mut(), || {
            match context.borrow().get_state() {
                pulse::context::State::Ready => Some(Ok(())),
                pulse::context::State::Failed |
                pulse::context::State::Terminated =>
                    Some(Err("Context state failed/terminated")),
                _ => None
            }
        }).unwrap();

        let introspect = Rc::new(RefCell::new(context.borrow().introspect()));

        PaApp {
            mainloop: mainloop,
            context: context,
            introspect: introspect
        }
    }

    pub fn wait_for<F>(&mut self, f: F) -> Result<(), &'static str>
    where F: Fn() -> Option<Result<(), &'static str>> {
        pa_wait_for(&mut self.mainloop.borrow_mut(), f)
    }

    pub fn wait_result<T: ?Sized>(&mut self, op: operation::Operation<T>) -> Result<(), &'static str> {
        pa_wait_result(&mut self.mainloop.borrow_mut(), op)
    }
}
