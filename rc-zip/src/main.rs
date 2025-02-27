use std::panic;

use owi_sym::{LazyArray, Symbolic};
use rc_zip::{chrono::offset, fsm::{ArchiveFsm, FsmResult}};

const BLOCK_SIZE: usize = 5;
const FILE_SIZE: usize = 30;

fn main() {
    // panic::set_hook(Box::new(|panic_info| {
    //     let s = panic_info.to_string();
    //     owi_sym::write_to_stdout(s.as_bytes());
    // }));

    let bytes : LazyArray<FILE_SIZE, u8> = owi_sym::LazyArray::new();
    let mut fsm = ArchiveFsm::new(FILE_SIZE as u64);
    let archive = 'read_zip: loop {
        if let Some(offset) = fsm.wants_read() {
            let offset : usize = offset.try_into().unwrap();
            let max_requested_len = FILE_SIZE - offset;
            let readable_len = max_requested_len.min(BLOCK_SIZE);
            let len = std::cmp::min(readable_len, fsm.space().len());
            fsm.space()[..len].copy_from_slice(&bytes.get_sub_slice(offset..offset + len).unwrap());
            match len {
                0 => panic!("EOF!"),
                read_bytes => {
                    fsm.fill(read_bytes);
                }
            }
        }

        fsm = match fsm.process() {
            Ok(res) => match res {
                FsmResult::Continue(fsm) => fsm,
                FsmResult::Done(archive) => break 'read_zip archive,
            },
            Err(_) => {
                owi_sym::stop_exploration()
            }
        }
    };

    let _ = archive;
}
