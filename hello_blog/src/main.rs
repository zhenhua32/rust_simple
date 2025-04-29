use hello_blog::Post;


fn main() {
    println!("Hello, world!");

    let mut post = Post::new();
    post.add_text("Hello, world!");
    assert_eq!(post.content(), "");

    post.request_review();
    assert_eq!(post.content(), "");

    post.approve();
    assert_eq!(post.content(), "Hello, world!");
}
