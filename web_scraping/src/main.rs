

//   =============================================
//    Web Scrapping Project
///   ============================================

use std::sync::{Arc};
use std::time::Instant;
use std::thread;
use ureq::AgentBuilder;

fn main() -> Result<(), ureq::Error> {
    let web_pages = vec![
        "https://github.com/Jethrolopwus/SmartCrib/blob/487f869aa8dac4a84a670ec2182e9cb549b569eb/frontend/lib/contracts/hooks.ts#L55",
        "https://github.com/Jethrolopwus/SmartCrib/blob/487f869aa8dac4a84a670ec2182e9cb549b569eb/frontend/lib/contracts/types.ts#L115",
        "https://github.com/Jethrolopwus/shadownet/blob/d4fa22975e119e1cb6c50b283e35ce0dfa6d8c38/contract/Scarb.lock#L14",
        "https://github.com/Jethrolopwus/attendance-tracker/blob/d4d13c38491827ca3d6bb3f0b4565fa0653b0b67/src/AttendanceTracker.sol#L11",
        "https://github.com/Jethrolopwus/bookmarks_api/blob/e6e578931dcdf22593cbb2274cc5a1c7fc5be777/src/auth/auth.module.ts#L8",
    ];

    let agent = AgentBuilder::new().build();

    // single-threaded fetch
    let now = Instant::now();
    for web_page in &web_pages {
        let _body = agent.get(web_page).call()?.into_string()?;
    }
    println!("Time taken without threads: {:.2?}", now.elapsed());

    // multi-threaded fetch
    let now = Instant::now();
    let agent = Arc::new(agent);

    let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new();

    for web_page in web_pages {
        let agent_thread = agent.clone();
        let url = web_page.to_string();

        let j = thread::spawn(move || {
            let _body = agent_thread.get(&url).call()?.into_string()?;
            Ok(())
        });

        handles.push(j);
    }

    for handle in handles {
        handle.join().unwrap().unwrap();
    }

    println!("Time taken using threads: {:.2?}", now.elapsed());
    Ok(())
}
