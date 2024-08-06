use std::mem::MaybeUninit;

use era_cudart_sys::{
    cudaOccupancyAvailableDynamicSMemPerBlock, cudaOccupancyMaxActiveBlocksPerMultiprocessor,
    cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,
};

use crate::execution::KernelFunction;
use crate::result::{CudaResult, CudaResultWrap};

pub fn available_dynamic_smem_per_block(
    kernel_function: &impl KernelFunction,
    num_blocks: i32,
    block_size: i32,
) -> CudaResult<usize> {
    let mut result = MaybeUninit::<usize>::uninit();
    unsafe {
        cudaOccupancyAvailableDynamicSMemPerBlock(
            result.as_mut_ptr(),
            kernel_function.as_ptr(),
            num_blocks,
            block_size,
        )
        .wrap_maybe_uninit(result)
    }
}

pub fn max_active_blocks_per_multiprocessor(
    kernel_function: &impl KernelFunction,
    block_size: i32,
    dynamic_smem_size: usize,
) -> CudaResult<i32> {
    let mut result = MaybeUninit::<i32>::uninit();
    unsafe {
        cudaOccupancyMaxActiveBlocksPerMultiprocessor(
            result.as_mut_ptr(),
            kernel_function.as_ptr(),
            block_size,
            dynamic_smem_size,
        )
        .wrap_maybe_uninit(result)
    }
}

pub fn max_active_blocks_per_multiprocessor_with_flags(
    kernel_function: &impl KernelFunction,
    block_size: i32,
    dynamic_smem_size: usize,
    flags: u32,
) -> CudaResult<i32> {
    let mut result = MaybeUninit::<i32>::uninit();
    unsafe {
        cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
            result.as_mut_ptr(),
            kernel_function.as_ptr(),
            block_size,
            dynamic_smem_size,
            flags,
        )
        .wrap_maybe_uninit(result)
    }
}
