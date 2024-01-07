use api::Request;

pub mod api;

fn main() {
    let my_path = Request::new(api::MethodType::Get)
        .resource(api::CollectionType::Posts)
        .id(10)
        .relation(api::CollectionType::Comments)
        .build()
        .fetch();
    println!("{:#?}", my_path.unwrap());

    let next_path = Request::new(api::MethodType::Get)
        .resource(api::CollectionType::Users)
        .query(api::QueryType::PostId, 10)
        .build()
        .fetch();
    println!("{:?}", next_path);
}
