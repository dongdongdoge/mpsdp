use doppio::client;
use doppio::server;
use doppio::schema;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize server
    let server = server::Server::new();
    let server_handle = tokio::spawn(async move {
        server.start().await;
    });

    // Initialize client
    let client = client::Client::new();

    // Submit some data points
    let data_points = vec![
        schema::DataPoint::new(vec![1.0, 2.0, 3.0]),
        schema::DataPoint::new(vec![4.0, 5.0, 6.0]),
        schema::DataPoint::new(vec![7.0, 8.0, 9.0]),
    ];

    for data in data_points {
        client.submit_data(data).await?;
    }

    // Execute a mean query
    let mean_query = schema::Query::new(
        schema::QueryType::Mean,
        vec!["feature1".to_string(), "feature2".to_string()],
    );
    let mean_result = client.execute_query(mean_query).await?;
    println!("Mean query result: {:?}", mean_result);

    // Execute a histogram query
    let hist_query = schema::Query::new(
        schema::QueryType::Histogram,
        vec!["feature1".to_string()],
    );
    let hist_result = client.execute_query(hist_query).await?;
    println!("Histogram query result: {:?}", hist_result);

    // Cleanup
    server_handle.abort();
    Ok(())
} 