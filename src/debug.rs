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

use crate::chunk;
use crate::value;

pub fn disassemble_chunk(chunk: &chunk::Chunk, name: &str) {
    println!("=== {} ===", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &chunk::Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction = chunk::OpCode::from(chunk.code[offset]);
    match instruction {
        chunk::OpCode::CONSTANT => constant_instruction("CONSTANT", chunk, offset),
        chunk::OpCode::RETURN => simple_instruction("RETURN", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &chunk::Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:16} {:4} '", name, constant);
    value::print_value(chunk.constants[constant as usize]);
    println!("'");
    offset + 2
}
