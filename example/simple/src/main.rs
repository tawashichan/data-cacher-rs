use data_cacher_rs;

#[tokio::main]
async fn main() {
    struct Fetcher {}

    #[derive(Debug)]
    struct E {

    }
    #[derive(Clone,Debug)]
    struct Data {
        a: i64,
    }

    use async_trait::async_trait;
 
    #[async_trait]
    impl data_cacher_rs::DataFetcher<Data> for Fetcher {
        type E = E;
        async fn fetch(&self) -> Result<Data,Self::E> {
            Ok(Data{a: 100})
        }
    }

    let fetcher = Fetcher{};

    let holder = std::sync::Arc::new(data_cacher_rs::CacheHolder::new(fetcher,1).await.unwrap());
    let value = holder.clone().read_data().await;
    println!("{:?}",value);
    holder.clone().rotate(); 

    std::thread::sleep(std::time::Duration::new(2, 0));
    let value2 = holder.clone().read_data().await;

    println!("{:?}",value2);

}
