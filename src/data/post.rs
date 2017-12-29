use json::Value;
use data::{Listing, Thing, Thread};
use failure::Error;
use App;

/// A struct that represents a submission to reddit
#[derive(Debug)]
pub struct Post {
	/// The comments on this post
	pub comments: Listing<Thread>,
}

impl Thing for Post {
	fn from_value(_data: &Value, _app: &App) -> Result<Post, Error> {
		/*let post = data[0]["data"]["children"][0];
		
		let mut comments = Vec::new();
		let comment_data= data[1]["data"]["children"];
		for comment in comment_data.as_array().unwrap() {
			comments.push(Thread::from_value(&comment, app)?)
		}
		
		Ok(Post {
			comments: comments,
			raw: data.clone()
		})*/
		unimplemented!()
	}
}
