use core::fmt::Debug;
use core::marker::PhantomData;

pub async fn api_call(query_param: &str) -> String {
    println!("Calling external API with: {}", query_param);
    format!("response for '{}'", query_param)
}

#[allow(dead_code)]
pub struct Stream<T: Send + 'static + Debug> {
    _pd: PhantomData<T>,
}

#[allow(dead_code)]
impl<T: Send + 'static + Debug> Stream<T> {
    pub fn new() -> Self {
        Stream { _pd: PhantomData }
    }

    pub async fn next(&self) -> Option<T> {
        unimplemented!("Read from persistent storage");
    }
}
