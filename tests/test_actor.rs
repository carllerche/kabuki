extern crate futures;
extern crate tokio_core;
extern crate kabuki;

use futures::Future;
use tokio_core::reactor;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn simple_request_response() {
    let mut core = reactor::Core::new().unwrap();

    let mut tx = kabuki::Builder::new()
        .spawn_fn(&core.handle(), |msg| {
            assert_eq!(msg, "ping");
            Ok::<&'static str, ()>("pong")
        });

    let resp = tx.call("ping");
    let resp = core.run(resp).unwrap();

    assert_eq!("pong", resp);
}

#[test]
fn multiple_request_response() {
    let mut core = reactor::Core::new().unwrap();
    let (ch_tx, rx) = mpsc::channel();

    let mut tx = kabuki::Builder::new()
        .spawn_fn(&core.handle(), move |msg| {
            ch_tx.send(msg).unwrap();
            Ok::<&'static str, ()>("pong")
        });

    let resp = tx.call("ping 1");
    let resp = core.run(resp).unwrap();

    assert_eq!("pong", resp);
    assert_eq!("ping 1", rx.recv().unwrap());

    let resp = tx.call("ping 2");
    let resp = core.run(resp).unwrap();

    assert_eq!("pong", resp);
    assert_eq!("ping 2", rx.recv().unwrap());
}

#[test]
fn actor_ref_is_clone_and_send() {
    let mut core = reactor::Core::new().unwrap();

    let mut tx = kabuki::Builder::new()
        .spawn_fn(&core.handle(), |msg| {
            Ok::<&'static str, ()>(msg)
        });

    let th = {
        let mut tx = tx.clone();
        thread::spawn(move || {
            tx.call("ping 1").wait().unwrap()
        })
    };

    thread::sleep(Duration::from_millis(100));

    let resp = tx.call("ping 2");
    let resp = core.run(resp).unwrap();

    assert_eq!("ping 2", resp);
    assert_eq!("ping 1", th.join().unwrap());
}
