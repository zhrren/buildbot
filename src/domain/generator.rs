use di::injectable;
use idgenerator_thin::{IdGeneratorOptions, YitIdHelper};

#[injectable]
pub struct Generator {}

impl Generator {
  pub fn configure(&self) {
    let mut options = IdGeneratorOptions::new(1);
    options.worker_id_bit_length = 1;
    options.base_time = 1710034500000;
    YitIdHelper::set_id_generator(options);
  }

  pub fn unique_id(&self) -> i64 {
    return YitIdHelper::next_id();
  }
}
