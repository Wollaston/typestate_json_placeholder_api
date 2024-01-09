pub mod api;

#[cfg(test)]
mod tests {
    use crate::api::{CollectionType, MethodType, Request};

    #[test]
    fn get_resource_users_url() {
        let url = Request::new(MethodType::Get)
            .resource(CollectionType::Users)
            .build()
            .url();
        assert_eq!(url, "https://jsonplaceholder.typicode.com/users");
    }

    #[test]
    fn get_comments_query_post_id_url() {
        let url = Request::new(MethodType::Get)
            .resource(CollectionType::Comments)
            .query(crate::api::QueryType::PostId, 1)
            .build()
            .url();
        assert_eq!(
            url,
            "https://jsonplaceholder.typicode.com/comments?postId=1"
        );
    }

    #[test]
    fn get_posts_id_url() {
        let url = Request::new(MethodType::Get)
            .resource(CollectionType::Posts)
            .id(1)
            .build()
            .url();
        assert_eq!(url, "https://jsonplaceholder.typicode.com/posts/1");
    }

    #[test]
    fn get_posts_id_comments_url() {
        let url = Request::new(MethodType::Get)
            .resource(CollectionType::Posts)
            .id(1)
            .relation(CollectionType::Comments)
            .build()
            .url();
        assert_eq!(url, "https://jsonplaceholder.typicode.com/posts/1/comments");
    }
}
