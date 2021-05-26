use hn_api::HnClient;

async fn print(api: &HnClient, items: &[u32]) {
    for id in items {
        let item = api.get_item(*id).await.unwrap().unwrap();
        let author = match item.author() {
            Some(username) => {
                let user = api.get_user(username).await.unwrap().unwrap();
                format!("{}, karma {}", username, user.karma)
            }
            None => "?".into(),
        };
        println!(
            "- {}: {} (by {})",
            item.id(),
            item.title().unwrap_or("?"),
            author
        )
    }
}

#[tokio::main]
async fn main() {
    println!("What's new on HN:\n");

    let api = HnClient::init().unwrap();

    let top = api.get_top_stories().await.unwrap();
    let new = api.get_new_stories().await.unwrap();
    let best = api.get_best_stories().await.unwrap();

    let count = 3;

    println!("Top {} stories:", count);
    print(&api, &top[..count]).await;

    println!("\nNewest count stories:");
    print(&api, &new[..count]).await;

    println!("\nBest count stories:");
    print(&api, &best[..count]).await;
}
