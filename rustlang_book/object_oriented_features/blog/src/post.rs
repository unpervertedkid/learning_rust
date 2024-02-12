pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Pending {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Pending {}

impl State for Pending {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_post_should_set_state_to_draft() {
        let mut post = Post::new();
        post.add_text("Hey, this is a blog post");

        assert_eq!("", post.content());
    }

    #[test]
    fn requesting_review_on_a_post_should_set_state_to_pending() {
        let mut post = Post::new();
        post.add_text("Hey, this is a blog post");
        post.request_review();

        assert_eq!("", post.content());
    }

    #[test]
    fn approving_a_review_should_set_state_to_published() {
        let mut post = Post::new();
        post.add_text("Hey, this is a blog post");
        post.request_review();
        post.approve();

        assert_eq!("Hey, this is a blog post", post.content());
    }
}
