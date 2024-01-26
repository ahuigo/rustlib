#[test]
fn trait_func(){
    // 定义trait 行为
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Weibo {
        pub username: String,
        pub content: String
    }
    
    // 实现trait 行为
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }
}