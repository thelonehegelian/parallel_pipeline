use crossbeam_channel::bounded;
use std::thread;
use std::time::Duration;

fn main() {
    let (snd1, rcv1) = bounded::<i32>(1);
    let (snd2, rcv2) = bounded::<i32>(1);
    let num_msgs = 4;
    let num_workers = 4;

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            for i in 0..num_msgs {
                snd1.send(i).unwrap();
                println!("Message sent {}", i);
            }
            // Close the channel - this is necessary to exit
            // the for-loop in the worker
            drop(snd1);
        });

        for i in 0..num_workers {
            let (sender, receiver) = (snd2.clone(), rcv2.clone());
            s.spawn(move |_| {
                for msg in receiver.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sender.send(msg).unwrap();
                }
            });
        }
        drop(snd2);

        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap()
}
