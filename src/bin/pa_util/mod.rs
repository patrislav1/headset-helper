use pulse::mainloop::standard::Mainloop;
use pulse::context::{introspect::Introspector, Context, FlagSet as ContextFlagSet};
use pulse::proplist::Proplist;
use pulse::mainloop::standard::IterateResult;
use pulse::operation;

pub struct PaApp {
    mainloop: Mainloop,
    context: Context,
    pub introspect: Introspector
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

        let mut mainloop = Mainloop::new()
            .expect("Failed to create mainloop");
    
        let mut context =
            Context::new_with_proplist(&mainloop, name /*+ "Context"*/, &proplist)
            .expect("Failed to create new context");
    
        context.connect(None, ContextFlagSet::NOFLAGS, None)
            .expect("Failed to connect context");
    
        pa_wait_for(&mut mainloop, || {
            match context.get_state() {
                pulse::context::State::Ready => Some(Ok(())),
                pulse::context::State::Failed |
                pulse::context::State::Terminated =>
                    Some(Err("Context state failed/terminated")),
                _ => None
            }
        }).unwrap();

        let introspect = context.introspect();

        PaApp {
            mainloop: mainloop,
            context: context,
            introspect: introspect
        }
    }

    pub fn wait_for<F>(&mut self, f: F) -> Result<(), &'static str>
    where F: Fn() -> Option<Result<(), &'static str>> {
        pa_wait_for(&mut self.mainloop, f)
    }

    pub fn wait_result<T: ?Sized>(&mut self, op: operation::Operation<T>) -> Result<(), &'static str> {
        pa_wait_result(&mut self.mainloop, op)
    }
}
