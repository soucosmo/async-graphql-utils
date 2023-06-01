use mongodb::{
    Client,
    Database,
    options::{
        ClientOptions
    },
};

pub struct ConnectDatabase {
    pub db: Database,
}

impl ConnectDatabase {
    pub async fn init(dsn: &String) -> Self {
        let client_options: ClientOptions = ClientOptions::parse(dsn).await.unwrap();

        let client: Client = Client::with_options(client_options).unwrap();

        let db: Database = client.default_database().unwrap();

        ConnectDatabase { db }
    }
}
