

// This file contains a deisgn pattern that is better for rust projects
// than the 'State Pattern' that is traditon (and used in post_example.rs)


fn main() {

	let mut post = Post::new();

	post.add_text("I ate a salad for lunch today");


	let post = post.request_review();

	let post = post.approve();

	assert_eq!("I ate a salad for lunch today", post.contents());
}


pub struct Post {
	content: String,
}

pub struct DraftPost {
	content: String,
}

impl Post {

	pub fn new() -> DraftPost {
		DraftPost {
			contents: String::new(),
		}
	}	

	pub fn content(&self) -> &str {
		&self.content
	}

}


impl DraftPost {

	pub fn add_text(&mut self, text: &str) {

		self.content.push_str(text);
	}

	pub n request_review(self) -> PendingReviewPost {
		PendingReviewPost {
			content: self.content,
		}
	}	

}


pub struct PendingReviewPost {
	content: String,
}

impl PendingReviewPost {

	pub fn approve(self) -> Post {
		Post {
			content: self.content,
		}
	}
}



