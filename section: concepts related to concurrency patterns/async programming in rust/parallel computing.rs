use tokio::task;

#[tokio::main]
async fn main() {
    let handle1 = task::spawn(async {
        // perform some expensive computation asynchronously
    });

    let handle2 = task::spawn(async {
        // perform some other expensive computation asynchronously
    });

    let (result1, result2) = tokio::join!(handle1, handle2);

    // combine the results of the two tasks
}
