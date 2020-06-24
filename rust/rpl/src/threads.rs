use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};


pub fn main(){
    let handle = thread::spawn(||{
        for i in 0..10{
            thread::sleep(Duration::from_millis(1));
            println!("{} in thread", i);
        }
    });
    for i in 0..5 {
        thread::sleep(Duration::from_millis(1));
        println!("{} in main", i);
    }
    handle.join().unwrap();

    let vec = vec![1,2,3,4,5];
    thread::spawn(move ||{
        for it in vec.iter(){
            println!("{}", *it)
        }
    }).join().unwrap();
/*
    let vec = vec![1,2,3,4,5];
    let refv = &vec;
    let handle = thread::spawn( ||{
        for it in refv.iter(){
            println!("{}", *it)
        }
    }).join().unwrap();
 */
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       tx.send(String::from("Hi"));
    }).join().unwrap();
    match rx.recv() {
        Ok(T) => println!("{}", T),
        Err(err) => println!("{}", err),
    }
    mpsc_test();
    mutex_test();
    mutex0_test();
}

fn mpsc_test(){
    let (tx,rx) = mpsc::channel();
    let txc = mpsc::Sender::clone(&tx);

    thread::spawn(move ||{
        let vals = vec![
            "hi","from","the","thread"
        ];
        for val in vals {
            tx.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move ||{
        let vals = vec![
            "hi","from","the","thread"
        ];
        for val in vals {
            txc.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for r in rx {
        println!("{}",r);
    }
}

fn mutex_test(){
    let m = Mutex::new(3);
    {
        let mut num = m.lock().unwrap();
        *num = 0;
    }
    println!("m = {:?}", m);
}

fn mutex0_test(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join();
    }
    println!("Result {}", *counter.lock().unwrap());
}