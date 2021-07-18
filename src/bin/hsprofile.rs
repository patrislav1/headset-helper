use pulse::mainloop::standard::Mainloop;
use pulse::context::{introspect, Context, FlagSet as ContextFlagSet};
use pulse::proplist::Proplist;
use pulse::mainloop::standard::IterateResult;
use pulse::operation;
use pulse::callbacks::ListResult;

fn sink_info_dumper(sil: ListResult<&introspect::SinkInfo>) {
    match sil {
        ListResult::Item(si) => {
            println!("index: {}", si.index);
            println!("name: {}", si.name.as_ref().unwrap());
            println!("desc: {}", si.description.as_ref().unwrap());
            println!("mute: {}", si.mute);
        },
        ListResult::End => {
            println!("List end.");
        },
        ListResult::Error => {
            eprintln!("Error while receiving list!");
        },
    }
}

fn main() {
    let mut proplist = Proplist::new().unwrap();
    proplist.set_str(pulse::proplist::properties::APPLICATION_NAME, "FooApp")
        .unwrap();

    let mut mainloop = Mainloop::new().expect("Failed to create mainloop");

    let mut context =
    Context::new_with_proplist(&mainloop, "FooAppContext", &proplist)
        .expect("Failed to create new context");

    context.connect(None, ContextFlagSet::NOFLAGS, None)
        .expect("Failed to connect context");

    // Wait for context to be ready
    loop {
        match mainloop.iterate(false) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }
        match context.get_state() {
            pulse::context::State::Ready => { break; },
            pulse::context::State::Failed |
            pulse::context::State::Terminated => {
                eprintln!("Context state failed/terminated, quitting...");
                return;
            },
            _ => {},
        }
    }

    let introspect = &context.introspect();
    let result = introspect.get_sink_info_list(sink_info_dumper);

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
