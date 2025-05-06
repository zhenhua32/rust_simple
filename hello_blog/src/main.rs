use hello_blog::Post;


fn main() {
    println!("Hello, world!");

    let mut post = Post::new();
    post.add_text("Hello, world!");
    let post = post.request_review();

    let post = post.approve();

    assert_eq!(post.content(), "Hello, world!");
}
