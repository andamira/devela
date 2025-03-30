// devela::examples::work::coro_manager
//!

use devela::{CoroManager, serr, sok};

fn main() {
    let mut cr = CoroManager::<char, String>::new();

    for i in 1..=3 {
        cr.push(|mut c| async move {
            println!("> instance {i} NEW");

            let mut count = 0;
            loop {
                let res = c.yield_ok('a').await?;
                println!("  instance {i} A.{count} {res:?})");

                let _res = c.yield_ok('b').await?;
                println!("  instance {i} B {_res:?}");

                if count > 2 {
                    break;
                }
                count += 1;
            }

            println!("  instance {i} BYE!");

            if i == 2 { serr(format!["instance {i} produced an error"]) } else { sok('x') }
        });
    }
    cr.push(|mut c| async move {
        println!("> instance 4 NEW");
        for _n in 0..3 {
            let _res = c.yield_err(format!["custom err"]).await?;
        }
        println!("  instance 4 BYE!");
        None
    });

    println!("Running");
    cr.run();
    println!("Done");
}
