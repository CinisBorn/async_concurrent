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
    });
    
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        
        let op1 = async move { 
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
    
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        
        let op2 = async {
            while let Some(message) = rx.recv().await {
                println!("Current message: {}", message);
            };
        };
        
        trpl::join(op1, op2).await;
    });
    
    println!("Program Finished!");
}
