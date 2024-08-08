use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("サーバーが起動しました。127.0.0.1:8080 でリッスンしています。");

    loop {
        let (socket, _) = listener.accept().await?;
        println!("新しいクライアントが接続しました。");
        // ここで接続処理を行います
    }
}
