use pulse::mainloop::standard::{Mainloop, IterateResult};
use pulse::context::{Context, FlagSet as ContextFlagSet};
use pulse::proplist::Proplist;
use pulse::operation;

fn main() {
    let mut proplist = Proplist::new().unwrap();
    proplist.set_str(
        pulse::proplist::properties::APPLICATION_NAME, "FooApp"
    ).unwrap();

    let mut mainloop = Mainloop::new()
        .expect("Failed to create mainloop");

    let mut context =
        Context::new_with_proplist(&mainloop, "FooAppCtx", &proplist)
        .expect("Failed to create new context");

    context.connect(None, ContextFlagSet::NOFLAGS, None)
        .expect("Failed to connect context");

    let introspect = context.introspect();

    let result = introspect.get_sink_info_list(|x| println!("cb: {:#?}", x));

    loop {
        match mainloop.iterate(false) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }
        match result.get_state() {
            operation::State::Done => { break; },
            operation::State::Cancelled => {
                eprintln!("Operation canceled, quitting...");
                return;
            },
            _ => {},
        }
    }
}
