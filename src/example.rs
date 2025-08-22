use crate::client::Client;
use crate::server::Server;
use crate::schema::{DataPoint, Query, QueryType};

pub fn simple_example() {
    let client = Client::new();
    let _server = Server::new();
    
    let _data = vec![
        DataPoint::new(vec![1.0, 2.0]),
        DataPoint::new(vec![3.0, 4.0]),
        DataPoint::new(vec![5.0, 6.0]),
    ];
    
    let query = Query::new(
        QueryType::Mean,
        vec!["feature1".to_string()],
    );
    
    let result = client.execute_query(query).unwrap();
    println!("Query result: {:?}", result.values());
}
