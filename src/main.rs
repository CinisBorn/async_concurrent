use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("First thread count: {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
    
        let handle2 = trpl::spawn_task(async {
            for i in 1..5 {
                println!("Second thread count: {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            };  
        });
        
        trpl::join(handle, handle2).await;
    })
}
