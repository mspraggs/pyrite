/* Copyright 2020 Matt Spraggs
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use yarel::chunk;
use yarel::class_store;
use yarel::error::ErrorKind;
use yarel::memory;
use yarel::object;
use yarel::vm;

fn repl(vm: &mut vm::Vm) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();

        match io::stdin().read_line(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    println!();
                    process::exit(0);
                }
                match vm::interpret(vm, buffer) {
                    Ok(_) => {}
                    Err(error) => eprint!("{}", error),
                }
            }
            _ => {
                eprintln!("Failed to read from stdin.");
                process::exit(74);
            }
        }
    }
}

fn run_file(vm: &mut vm::Vm, path: &str) {
    let source = fs::read_to_string(path);
    let result = match source {
        Ok(contents) => vm::interpret(vm, contents),
        _ => panic!("Unable to read from file."),
    };

    if let Err(error) = result {
        let exit_code = if error.get_kind() == ErrorKind::CompileError {
            65
        } else {
            70
        };
        eprint!("{}", error);
        process::exit(exit_code);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let heap = memory::new_heap();
    let string_store = object::new_obj_string_store(&mut heap.borrow_mut());
    let chunk_store = chunk::new_chunk_store();
    let class_store =
        class_store::new_class_store(heap.clone(), string_store.clone(), chunk_store.clone());
    let mut vm = vm::new_root_vm_with_built_ins(heap, string_store, chunk_store, class_store);

    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        run_file(&mut vm, &args[1]);
    } else {
        eprintln!("Usage: ./yarel-cli [path]");
        process::exit(64);
    }
}