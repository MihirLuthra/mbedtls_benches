mod cli_args;
mod operation;

use std::{time::SystemTime, sync::Arc};
use cli_args::{Args, KeyType};
use structopt::StructOpt;
use operation::{Operation, OperationType};

use mbedtls::pk::Pk;
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

fn benchmark<B, O, A, R: 'static>(operation_info: Operation<B, O, A>, thread_count: u32, num_ops: u32)
where
    B: Fn() -> R, B: Send + Sync + 'static,
    O: Fn(&mut R), O: Send + Sync + 'static,
    A: Fn(&mut R), A: Send + Sync + 'static,
{
    let mut join_handles = Vec::with_capacity(thread_count as usize);

    println!("Performing {} {} operations", thread_count * num_ops, operation_info.operation_type);

    let start_time = SystemTime::now();

    let operation_info = Arc::new(operation_info);

    for thread_no in 1..=thread_count {
        let operation_info = operation_info.clone();
        let j = std::thread::Builder::new()
            .name(format!("thread-{}", thread_no))
            .spawn(move || {
                println!("Thread {}: Started", thread_no);
                let mut state = (operation_info.before)();
                for _ in 1..=num_ops {
                    (operation_info.operation)(&mut state);
                }
                (operation_info.after)(&mut state);
                println!("Thread {}: Done", thread_no);
            })
            .unwrap();

        join_handles.push(j);
    }

    for j in join_handles {
        j.join().unwrap();
    }

    let end_time = SystemTime::now();

    let time_difference = end_time.duration_since(start_time).unwrap().as_secs_f32();

    let operation_per_sec = (num_ops * thread_count) as f32 / time_difference;

    println!("Speed: {} {}/s", operation_per_sec, operation_info.operation_type);
}
