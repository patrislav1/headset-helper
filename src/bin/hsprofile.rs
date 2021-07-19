use pulse::mainloop::standard::Mainloop;
use pulse::context::{introspect, Context, FlagSet as ContextFlagSet};
use pulse::proplist::Proplist;
use pulse::mainloop::standard::IterateResult;
use pulse::operation;
use pulse::callbacks::ListResult;

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

fn source_info_dumper(sil: ListResult<&introspect::SourceInfo>) {
    match sil {
        ListResult::Item(si) => {
            if let Some(_) = si.monitor_of_sink {
                // Ignore monitors
                return;
            }
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

    pa_wait_for(&mut mainloop, || {
        match context.get_state() {
            pulse::context::State::Ready => Some(Ok(())),
            pulse::context::State::Failed |
            pulse::context::State::Terminated =>
                Some(Err("Context state failed/terminated")),
            _ => None
        }
    }).unwrap();

    let introspect = &context.introspect();

    println!("sources:");
    pa_wait_result(&mut mainloop,
        introspect.get_source_info_list(source_info_dumper)
    ).unwrap();

    println!("sinks:");
    pa_wait_result(&mut mainloop,
        introspect.get_sink_info_list(sink_info_dumper)
    ).unwrap();
}
