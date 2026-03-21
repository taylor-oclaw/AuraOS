extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_exit() {
    // Cleanup logic for the module
}

pub struct TwitterUser {
    username: String,
    followers_count: u32,
    following_count: u32,
    tweets: Vec<String>,
}

impl TwitterUser {
    pub fn new(username: &str) -> Self {
        TwitterUser {
            username: String::from(username),
            followers_count: 0,
            following_count: 0,
            tweets: Vec::new(),
        }
    }

    pub fn add_follower(&mut self) {
        self.followers_count += 1;
    }

    pub fn remove_follower(&mut self) {
        if self.followers_count > 0 {
            self.followers_count -= 1;
        }
    }

    pub fn follow_user(&mut self) {
        self.following_count += 1;
    }

    pub fn unfollow_user(&mut self) {
        if self.following_count > 0 {
            self.following_count -= 1;
        }
    }

    pub fn post_tweet(&mut self, tweet: &str) {
        self.tweets.push(String::from(tweet));
    }

    pub fn get_tweets(&self) -> &[String] {
        &self.tweets
    }
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_create_user(username: *const u8, username_len: usize) -> *mut TwitterUser {
    let username_slice = unsafe { core::slice::from_raw_parts(username, username_len) };
    let username_str = String::from_utf8_lossy(username_slice).into_owned();
    Box::leak(Box::new(TwitterUser::new(&username_str)))
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_add_follower(user: *mut TwitterUser) {
    unsafe { (*user).add_follower() };
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_remove_follower(user: *mut TwitterUser) {
    unsafe { (*user).remove_follower() };
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_follow_user(user: *mut TwitterUser) {
    unsafe { (*user).follow_user() };
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_unfollow_user(user: *mut TwitterUser) {
    unsafe { (*user).unfollow_user() };
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_post_tweet(user: *mut TwitterUser, tweet: *const u8, tweet_len: usize) {
    let tweet_slice = unsafe { core::slice::from_raw_parts(tweet, tweet_len) };
    let tweet_str = String::from_utf8_lossy(tweet_slice).into_owned();
    unsafe { (*user).post_tweet(&tweet_str) };
}

#[no_mangle]
pub extern "C" fn rel_twitter_enrich_get_tweets(user: *const TwitterUser, tweets: *mut *const u8, tweet_lengths: *mut usize, count: *mut usize) {
    let user_ref = unsafe { &*user };
    let tweets_vec = user_ref.get_tweets();
    let num_tweets = tweets_vec.len().min(unsafe { *count });
    for i in 0..num_tweets {
        let tweet_str = &tweets_vec[i];
        unsafe {
            (*tweets.offset(i as isize)) = tweet_str.as_ptr();
            (*tweet_lengths.offset(i as isize)) = tweet_str.len();
        }
    }
    unsafe { *count = num_tweets };
}
