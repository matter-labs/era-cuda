// error handling
// https://docs.nvidia.com/cuda/cuda-runtime-api/group__CUDART__ERROR.html

use era_cudart_sys::*;

pub fn get_last_error() -> CudaError {
    unsafe { cudaGetLastError() }
}

pub fn peek_at_last_error() -> CudaError {
    unsafe { cudaPeekAtLastError() }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn get_last_error_equals_success() {
        let result = get_last_error();
        assert_eq!(result, CudaError::Success)
    }

    #[test]
    #[serial]
    fn peek_at_last_error_equals_success() {
        let result = peek_at_last_error();
        assert_eq!(result, CudaError::Success)
    }
}
