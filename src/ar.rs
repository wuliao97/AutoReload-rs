use tokio::sync::Mutex;
use std::time::Duration;
use async_trait::async_trait;
use daemonize::Daemonize;
use daemonaize;

pub type Error = Box<dyn std::error::Error + Sync + Send>;

#[async_trait]
pub trait ARTrait {
    async fn execute(&self) -> Result<(), Error>;
}

pub struct AR<T: ARTrait> {
    obj: Mutex<T>,
    span: Duration
}
impl<T: ARTrait> AR<T> {
    pub fn new(obj: T, secs: u64) -> Self {
        env_logger::init();

        let obj = Mutex::new(obj);
        let span = Duration::from_secs(secs);
        Self {
            obj, span
        }
    }

    pub fn new_thirty_mins(obj: T) -> Self {
        let secs = 60 * 30;
        AR::new(obj, secs)
    } 

    pub fn new_one_hour(obj: T) -> Self {
        let secs = 60 * 60;
        AR::new(obj, secs)
    } 


    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let x = self.obj.lock().await;

        log::debug!("Starting to Auto Reload");
        println!("test");


        let daemonize = Daemonize::new()
            .privileged_action(|| return "Executed before drop privileges");

        match daemonize.start() {
            Ok(_) => {
                x.execute().await;
                log::debug!("SUCCESS")
            }
            Err(err) => {
                log::error!("{:#?}", err)
            }
        }

        Ok(())
    }
}