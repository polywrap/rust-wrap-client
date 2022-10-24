use std::sync::{Arc, Mutex};

use wasmtime::{Memory, StoreContext, StoreContextMut};

pub fn write_to_memory(
    memory: Arc<Mutex<Memory>>,
    store_ctx: StoreContextMut<'_, u32>,
    offset: usize,
    data: &[u8],
) {
    let memory_guard = memory.lock().unwrap();
    memory_guard.data_mut(store_ctx)[offset..offset + data.len()].copy_from_slice(data);
}

pub fn read_from_memory(
    memory: Arc<Mutex<Memory>>,
    store_ctx: StoreContext<'_, u32>,
    offset: usize,
    length: usize,
) -> Vec<u8> {
    let memory_guard = memory.lock().unwrap();
    memory_guard.data(store_ctx)[offset..offset + length].to_vec()
}
