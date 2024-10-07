use textlargetxt::calculate;
#[tokio::main]
async fn main() {
    //測試 calculate 函數
    // 你可以在這裡使用 await
    calculate("100".to_string()).await;
}
