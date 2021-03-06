extern crate env_logger;
extern crate futures;
extern crate log;
extern crate rusty_tarantool;
extern crate tokio;

extern crate rmpv;
extern crate rmp_serde;
extern crate serde;
extern crate rmp;

use tokio::runtime::current_thread::Runtime;
use rusty_tarantool::tarantool::{ClientConfig};
use futures::{Future};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    println!("Simple client run!");
    let mut rt = Runtime::new().unwrap();

    let addr = "127.0.0.1:3301".parse().unwrap();
    let client = ClientConfig::new(addr,"rust","rust").build();
    let start = Instant::now();
    let count = 1000000;


    for x in 0..count {
        let resp = client.call_fn("test", &(("aa", "aa"), x))
            .and_then(move |response| {
                let s: (Vec<String>, Vec<u64>, Vec<Option<u64>>) = response.decode()?;
                let v = COUNTER.fetch_add(1, Ordering::SeqCst);
                if v==count-1 {
                    println!("All finished res={:?}", s);
                    let elapsed = start.elapsed();
                    // debug format:
                    println!("{:?}", elapsed);
                    std::process::exit(0);
                }
                Ok(())
            })
            .map_err(|_e| ())
        ;
        rt.spawn(resp);
    };

    let _res = rt.run();
}
