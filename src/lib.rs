

use tokio::{sync::RwLock};
use std::sync::Arc;
use async_trait::async_trait;
use std::marker::{Send,Sync};


pub struct CacheHolder<D: Send + Sync,F: DataFetcher<D> + Send + Sync> {
    // use Arc to avoid cloning D
    data: Arc<RwLock<Arc<Option<D>>>>,
    fetcher: F,
    interval: u64
}

#[async_trait]
pub trait DataFetcher<D> {
    type E;
    async fn fetch(&self) -> Result<D,Self::E>; 
}

impl <D: 'static + Send + Sync,F: 'static + DataFetcher<D> + Send + Sync> CacheHolder<D,F> {

    pub async fn new(fetcher: F,interval: u64) -> Result<Self,F::E> {
        let rorator = CacheHolder{
            data: Arc::new(RwLock::new(Arc::new(None))),
            fetcher: fetcher,
            interval,
        };
        //rorator.update_data().await?;
        Ok(rorator)
    }

    pub fn rotate(self: Arc<Self>) -> () {
        tokio::spawn(async move {
            loop {
                let _ = tokio::time::sleep(std::time::Duration::new(self.interval, 0)).await;
                self.update_data().await;
            }
        });
    }

    pub async fn read_data(&self) -> Arc<Option<D>> {
        let data = self.data.read().await;
        (*data).clone()
    }

    async fn update_data(&self) -> Result<(),F::E> {
        let new_data = self.fetcher.fetch().await?;
        let mut data = self.data.write().await;
        *data = Arc::new(Some(new_data));
        Ok(())
    }
    
}


#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct E {

    }

    #[tokio::test]
    async fn it_works() ->  std::result::Result<(),E>{

        struct Fetcher {}

        #[derive(Clone,Debug)]
        struct Data {
            a: i64,
        }

        use async_trait::async_trait;
     
        #[async_trait]
        impl crate::DataFetcher<Data> for Fetcher {
            type E = E;
            async fn fetch(&self) -> Result<Data,Self::E> {
                Ok(Data{a: 100})
            }
        }

        let fetcher = Fetcher{};

        let holder = std::sync::Arc::new(crate::CacheHolder::new(fetcher,1).await?);
        let value = holder.clone().read_data().await;
        println!("{:?}",value);
        holder.clone().rotate(); 

        std::thread::sleep(std::time::Duration::new(2, 0));
        let value2 = holder.clone().read_data().await;

        println!("{:?}",value2);

        Ok(())
    }
}
