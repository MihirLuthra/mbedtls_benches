mod cli_args;
mod operation;

use std::sync::Barrier;
use std::time::Duration;
use std::{time::SystemTime, sync::Arc};
use cli_args::{Args, KeyType};
use structopt::StructOpt;
use operation::{Operation, OperationType};

use mbedtls::pk::{Pk, EcGroupId};
use mbedtls::hash::Type as MdType;
use mbedtls_benches::{create_ec_from_curve, dummy_ec_sign, create_rsa_from_size, dummy_rsa_sign};

fn main() {
    let opt = Args::from_args();

    match opt.operation_type {
        operation::OperationType::Sign => {
            match opt.key_type {
                KeyType::Rsa => {
                    let before = move || {
                        create_rsa_from_size(opt.key_size)
                    };

                    let operation = move |pk: &mut Pk| {
                        dummy_rsa_sign(pk, opt.md_type)
                    };

                    let after = |_pk: &mut Pk| {};

                    let operation = Operation {
                        operation_type: OperationType::Sign,
                        before,
                        operation,
                        after,
                    };

                    benchmark(operation, opt.thread_count, opt.num_ops);
                },
                KeyType::Ecdsa => {
                    let before = move || {
                        create_ec_from_curve(opt.curve)
                    };

                    let operation = move |pk: &mut Pk| {
                        dummy_ec_sign(pk, opt.md_type)
                    };

                    let after = |_pk: &mut Pk| {};

                    let operation = Operation {
                        operation_type: OperationType::Sign,
                        before,
                        operation,
                        after,
                    };

                    benchmark(operation, opt.thread_count, opt.num_ops);
                },
            };
        },
    };

}

fn benchmark<B, O, A, R: 'static>(operation: Operation<B, O, A>, thread_count: u32, num_ops: u32)
where
    B: Fn() -> R, B: Send + Sync + 'static,
    O: Fn(&mut R), O: Send + Sync + 'static,
    A: Fn(&mut R), A: Send + Sync + 'static,
{
    let mut join_handles = Vec::with_capacity(thread_count as usize);

    println!("Performing {} {} operations", thread_count * num_ops, operation.operation_type);


    let operation = Arc::new(operation);

    // barrier for all threads to warm up and execute `operation.before`
    // +1 for the spawner.
    let thread_start_barrier = Arc::new(Barrier::new(thread_count as usize + 1));
    let thread_end_barrier = Arc::new(Barrier::new(thread_count as usize + 1));
    let timer_start_barrier = Arc::new(Barrier::new(thread_count as usize + 1));

    for thread_no in 1..=thread_count {

        let operation = operation.clone();
        let thread_start_barrier = thread_start_barrier.clone();
        let thread_end_barrier = thread_end_barrier.clone();
        let timer_start_barrier = timer_start_barrier.clone();

        let j = std::thread::Builder::new()
            .name(format!("thread-{}", thread_no))
            .spawn(move || {
                println!("Thread {}: Warm up", thread_no);
                warm_up();

                let mut state = (operation.before)();

                thread_start_barrier.wait();
                timer_start_barrier.wait();

                println!("Thread {}: Started", thread_no);

                for _ in 1..=num_ops {
                    (operation.operation)(&mut state);
                }

                println!("Thread {}: Done", thread_no);

                thread_end_barrier.wait();

                (operation.after)(&mut state);
            })
            .unwrap();

        join_handles.push(j);
    }

    thread_start_barrier.wait();
    println!();

    let start_time = SystemTime::now();

    timer_start_barrier.wait();

    thread_end_barrier.wait();

    let end_time = SystemTime::now();

    for j in join_handles {
        j.join().unwrap();
    }

    let time_difference = end_time.duration_since(start_time).unwrap().as_secs_f32();

    let operation_per_sec = (num_ops * thread_count) as f32 / time_difference;

    println!("Speed: {} {}/s", operation_per_sec, operation.operation_type);
}

fn warm_up() {
    const WARM_UP_CRYPTO_OPS: u32 = 100;

    let mut ec = create_ec_from_curve(EcGroupId::SecP256R1);
    let mut rsa = create_rsa_from_size(1024);

    for _ in 1..=WARM_UP_CRYPTO_OPS {
        dummy_ec_sign(&mut ec, MdType::Sha256);
        dummy_rsa_sign(&mut rsa, MdType::Sha256);
    }

    std::thread::sleep(Duration::from_secs(3));
}
