/*use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a warp filter that captures the requested path and prints it
    let log_request = warp::path::tail()
        .map(|path: warp::filters::path::Tail| {
            println!("Request received: /{}", path.as_str());
            warp::reply::html("Hello, warp!")
        });

    // Start the warp server
    warp::serve(log_request)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
*/

fn main()
{
    println!("aaa");
}