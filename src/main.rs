use api::Request;

pub mod api;

fn main() {
    let my_path = Request::new(api::MethodType::Get)
        .resource(api::CollectionType::Posts)
        .id(1)
        .relation(api::CollectionType::Comments)
        .build()
        .fetch();
    println!("{:#?}", my_path.unwrap());
}
