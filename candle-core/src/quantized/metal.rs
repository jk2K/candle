use crate::{Result, MetalDevice};
use super::{QuantizedType, GgmlDType};

pub struct QStorage;

impl QuantizedType for QStorage{
    fn dtype(&self) -> GgmlDType{
        todo!();
    }

    fn matmul_t(&self, mkn: (usize, usize, usize), lhs: &[f32], dst: &mut [f32]) -> Result<()>{
        todo!();
    }

    fn to_float(&self, ys: &mut [f32]) -> Result<()>{
        todo!();
    }

    fn storage_size_in_bytes(&self) -> usize{
        todo!();
    }

    fn as_ptr(&self) -> *const u8{
        todo!();
    }

    fn block_size(&self) -> usize{
        todo!();
    }
}

pub fn load_quantized_metal<T: super::GgmlType + Send + Sync + 'static>(device: &MetalDevice, data: &[T]) -> Result<Box<QStorage>>{
    todo!("Implement the load");
}
