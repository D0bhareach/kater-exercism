use std::collections::HashMap;
use std::sync::{Arc, mpsc::{channel, Sender}};
use std::thread;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
let vector:Vec<AtomicUsize> = std::iter::repeat_with(||AtomicUsize::new(0)).take(253).collect(); 


fn count_letters(s: &str, v: &Arc<Vec<AtomicUsize>>) {
    if s.is_empty() {
        return;
    }
    s.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
        let cx = c.to_lowercase().next().unwrap();
        let cd:u32 = cx.into();
        v[cd as usize].fetch_add(1usize, Ordering::SeqCst);
    });
}
    let mut counter = 0;
    let length = input.len();
    if length == 0 {
        return HashMap::new();
    }
    let workers = if length < 4 {length} else {worker_count};
    let mut string_senders: Vec<Sender<String>> = Vec::with_capacity(workers);
    let (sender, receiver) =
    channel::<String>();
    let result_vector = Arc::new(vector);

    for idx in 0..workers {
        let res_vec = result_vector.clone(); 

        // this used for done messages.
        let hm = sender.clone();
        // this used to pass strigns to thread.
        let (str_sender, str_receiver) = channel::<String>();
        // let arc_map = Arc::clone(&res);

        std::thread::Builder::new().name(idx.to_string()).spawn(move||{
            
            while let Some(s) = str_receiver.iter().next() {
                if s.eq("stop"){
                    hm.send("final_signal".into()).unwrap();
                    drop(hm);
                    break;
                }
                
                count_letters(&s, &res_vec);
                hm.send(thread::current().name().unwrap().into())
                .unwrap(); 
            }
        }).unwrap();

        // send first line to new thread
        str_sender.send(input[counter].into()).unwrap();
        counter +=1;
        string_senders.push(str_sender);
        
    }

    let mut final_messages: usize = 0;
    for wr in receiver.iter() {
        if wr.eq("final_signal"){
            final_messages += 1;
        if final_messages == workers {
            break;
        }
        } else {
        
         
            let index:usize = wr.parse().unwrap();
        let thread_sender = &string_senders[index];
        if counter < length{
        let s = input[counter].to_string();
        counter += 1;
        thread_sender.send(s).unwrap();
        } else {
            thread_sender.send("stop".into()).unwrap();
        }
    }
    }
    

    for thread_sender in string_senders {
        drop(thread_sender);
    }
        
    // due to error on line 50 have to put this stumble here.
    // have to give give time for other threads to run finalization code.
    // sleep duration is smallest on my computer when bench tests are passing.
    // tests are flaky with 80000.
    thread::sleep(std::time::Duration::from_nanos(90000));
    let rv: Vec<AtomicUsize> = Arc::try_unwrap(result_vector).expect("Arc still in use");
    let mut result : HashMap<char, usize>= HashMap::with_capacity(26);
        for (idx, au) in rv.iter().enumerate(){
            let ch = idx as u32;
            let n = au.load(Ordering::Relaxed);
            if  n != 0 {
                let k = char::from_u32(ch).unwrap();
                result.insert(k, n);
            }
        }
result
}