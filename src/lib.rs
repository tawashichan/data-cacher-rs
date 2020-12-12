

use tokio::{sync::RwLock};
use std::sync::Arc;
use async_trait::async_trait;

struct CacheHolder<D,M: ResponseMapper<D>> {
    endpoint: &'static str,
    // use Arc to avoid cloning D
    data: RwLock<Arc<Option<D>>>,
    mapper: M,
    interval: u64
}

pub trait ResponseMapper<Data>{
    type Resp;
    fn to_data(resp: Self::Resp) -> Data;
}

#[async_trait]
pub trait DataFetcher<D: for<'de> serde::Deserialize<'de>,E> {
    async fn fetch() -> Result<D,E>; 
}

impl <D: for<'de> serde::Deserialize<'de>,M: ResponseMapper<D>> CacheHolder<D,M> {

    pub async fn new(endpoint: &'static str,mapper: M,interval: u64) -> Result<Self,reqwest::Error> {
        let mut rorator = CacheHolder{
            endpoint,
            data: RwLock::new(Arc::new(None)),
            mapper,
            interval,
        };
        rorator.update_data().await?;
        Ok(rorator)
    }

    pub async fn rotate(&mut self) -> Result<Self,reqwest::Error> {
        loop {
            let _ = tokio::time::delay_for(std::time::Duration::new(self.interval, 0));
            self.update_data().await?;
        }
    }

    // ここcloneせずにいい感じにやるほうほうあるんだろうか...
    pub async fn read_data(&self) -> Arc<Option<D>> {
        self.data.read().await.clone()
    }

    async fn update_data(&mut self) -> Result<(),reqwest::Error> {
        let new_data = self.get_new_data().await?;
        let mut data = self.data.write().await;
        *data = Arc::new(Some(new_data));
        Ok(())
    }

    async fn get_new_data(&self) -> Result<D,reqwest::Error> {
        let resp = reqwest::get(self.endpoint).await?;
        let resp = resp.json().await?;
        Ok(resp)
    }
    
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
