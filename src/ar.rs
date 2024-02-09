use tokio::sync::Mutex;
use std::time::Duration;
use async_trait::async_trait;


#[async_trait]
pub trait ARTrait {
    async fn execute(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>>;
}

pub struct AR<T: ARTrait> {
    obj: Mutex<T>,
    span: Duration
}
impl<T: ARTrait> AR<T> {
    pub fn new(obj: T, secs: u64) -> Self {
        let obj = Mutex::new(obj);
        let span = Duration::from_secs(secs);
        Self {
            obj, span
        }
    }

    pub fn new_one_hour(obj: T) -> Self {
        let secs = 60 * 60;
        AR::new(obj, secs)
    } 


    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {

    

        Ok(())
    }
}