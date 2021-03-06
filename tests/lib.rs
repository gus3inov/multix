extern crate multix;

use multix::ThreadPool;
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[test]
fn one_thread() {
    let pool = ThreadPool::new(1);
    let (tx, rx) = mpsc::sync_channel(0);

    pool.send(move || {
        tx.send("lol").unwrap();
    })
    .unwrap();

    assert_eq!("lol", rx.recv().unwrap());
}

#[test]
fn two_thread() {
    let pool = ThreadPool::new(2);
    let (tx, rx) = mpsc::sync_channel(0);

    for _ in 0..2 {
        let tx = tx.clone();
        pool.send(move || {
            tx.send("lol").unwrap();
            thread::sleep(Duration::from_millis(500));

            tx.send("kek").unwrap();
            thread::sleep(Duration::from_millis(500));
        })
        .unwrap();
    }

    for &msg in ["lol", "lol", "kek", "kek"].iter() {
        assert_eq!(msg, rx.recv().unwrap());
    }
}

#[test]
fn clone_pool() {
    let pool = ThreadPool::new(1);
    let (tx, rx) = mpsc::sync_channel(1);

    pool.clone()
        .send(move || {
            tx.send("hey").unwrap();
        })
        .unwrap();

    assert_eq!("hey", rx.recv().unwrap());
}

#[test]
fn two_thread_job_on() {
    let pool = ThreadPool::new(2);
    let (tx, rx) = mpsc::sync_channel(0);

    for _ in 0..4 {
        let tx = tx.clone();

        pool.send(move || {
            tx.send("lol").unwrap();
            thread::sleep(Duration::from_millis(500));

            tx.send("kek").unwrap();
            thread::sleep(Duration::from_millis(500));
        })
        .unwrap();
    }

    for &msg in ["lol", "lol", "kek", "kek", "lol", "lol", "kek", "kek"].iter() {
        assert_eq!(msg, rx.recv().unwrap());
    }
}

#[test]
fn threads_shutdown_drop() {
    let pool = ThreadPool::single_thread();
    let atom = Arc::new(AtomicUsize::new(5));

    for _ in 0..10 {
        let atom = atom.clone();
        pool.send(move || {
            atom.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    }

    pool.close();

    assert!(pool.is_terminating() || pool.is_terminated());

    pool.await_termination();

    assert_eq!(15, atom.load(Ordering::SeqCst));
    assert!(pool.is_terminated());
}

// #[test]
// fn threads_shutdown_now() {
//     let pool = ThreadPool::single_thread();
//     let atom = Arc::new(AtomicUsize::new(0));

//     for _ in 0..10 {
//         let atom = atom.clone();
//         match pool.try_send(move || {
//             atom.fetch_add(1, Ordering::SeqCst);
//         }) {
//             Ok(_) => continue,
//             Err(_) => break,
//         };
//     }

//     pool.close_force();

//     assert!(pool.is_terminating() || pool.is_terminated());

//     pool.await_termination();

//     let is_not_filled = atom.load(Ordering::SeqCst) != 10;
//     assert!(is_not_filled);
//     assert!(pool.is_terminated());
// }

// #[test]
// fn mount_thread_hook() {
//     let (tx, rx) = mpsc::sync_channel(0);

//     let tx_mount = tx.clone();
//     let mount_thread = move || {
//         tx_mount.send("mounted").unwrap();
//     };
//     let (sender, _) = ThreadPool::new_with_hooks(1, mount_thread, || {});

//     sender
//         .send(move || {
//             tx.send("hey").unwrap();
//         })
//         .unwrap();

//     for &msg in ["mounted", "hey"].iter() {
//         assert_eq!(msg, rx.recv().unwrap());
//     }
// }

// #[test]
// fn unmount_thread_hook() {
//     let (tx, rx) = mpsc::sync_channel(0);

//     let tx_mount = tx.clone();
//     let mount_thread = move || {
//         tx_mount.send("mounted").unwrap();
//     };
//     let tx_unmount = tx.clone();
//     let unmount_thread = move || {
//         tx_unmount.send("unmounted").unwrap();
//     };
//     let (sender, _) = ThreadPool::new_with_hooks(1, mount_thread, unmount_thread);

//     sender
//         .send(move || {
//             tx.send("hey").unwrap();
//         })
//         .unwrap();
//     drop(sender);

//     for &msg in ["mounted", "hey", "unmounted"].iter() {
//         assert_eq!(msg, rx.recv().unwrap());
//     }
// }
