use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::Result;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use win_event_hook::events::{Event, NamedEvent};

fn main() -> Result<()> {
    // setup tracing for good measure
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // create our hook config
    let config = win_event_hook::Config::builder()
        .skip_own_process()
        .with_dedicated_thread()
        .with_events(vec![
            // to see these, try right clicking
            Event::Named(NamedEvent::ObjectShow),
            Event::Named(NamedEvent::ObjectHide),
            // to see this, try moving around the cursor
            Event::Named(NamedEvent::ObjectLocationChange),
        ])
        .finish();

    // and our handler
    let handler = |ev, _, _, _, _, _| {
        println!("got event: {:?}", ev);
    };

    // install the hook
    let mut hook = win_event_hook::WinEventHook::install(config, handler)?;

    // setup ctrlc to help us shutdown neatly when the user hits ctrl+c
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // wait for ctrl+c
    while running.load(Ordering::SeqCst) {}

    // uninstall the hook
    hook.uninstall()?;

    // exit cleanly
    Ok(())
}
