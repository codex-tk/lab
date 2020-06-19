use tokio::net::TcpListener;
use futures::stream::StreamExt;


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();
    let server_loop = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await{
            match socket_res{
                Ok(mut socket) =>{
                    tokio::spawn(async move{
                        let (mut r, mut w) = socket.split();
                        match tokio::io::copy(&mut r, &mut w).await{
                            Ok(amt) => println!("{} wrote", amt),
                            Err(_err) =>{}
                        };
                    });
                }
                Err(err)=>{
                    println!("{}", err);
                }
            }
        }
    };

    println!("Hello, world!");
    server_loop.await;
}
