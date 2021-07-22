use pulse::context::introspect;
use pulse::callbacks::ListResult;

mod pa_util;

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
    let mut pa = pa_util::PaApp::new("hsprofile");

    println!("sources:");
    let op1 = pa.introspect.borrow().get_source_info_list(source_info_dumper);
    pa.wait_result(
        op1
    ).unwrap();

    println!("sinks:");
    let op2 = pa.introspect.borrow().get_sink_info_list(sink_info_dumper);
    pa.wait_result(
        op2
    ).unwrap();

    println!("Done.");
}
