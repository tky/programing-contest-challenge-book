struct Task {
    id: String,
    start: u32,
    end: u32,
}

fn main() {
    let tasks = vec![
        Task {
            id: "1".to_string(),
            start: 1,
            end: 3,
        },
        Task {
            id: "2".to_string(),
            start: 2,
            end: 5,
        },
        Task {
            id: "3".to_string(),
            start: 4,
            end: 7,
        },
        Task {
            id: "4".to_string(),
            start: 6,
            end: 9,
        },
        Task {
            id: "5".to_string(),
            start: 8,
            end: 10,
        },
    ];
    let results = solve(&tasks);
    let job_str = results.iter().map(|task| task.id.to_string());
    println!("{}", results.len());
    println!("{}", job_str.collect::<Vec<String>>().join(" "));
}

fn solve(tasks: &[Task]) -> Vec<&Task> {
    let mut current_time = 0;
    let mut results = Vec::new();
    while let Some(task) = find_next_task(&tasks, current_time) {
        results.push(task);
        current_time = task.end;
    }
    results
}

// find the next task that can be executed and which ends the earliest
fn find_next_task(tasks: &[Task], current_time: u32) -> Option<&Task> {
    tasks
        .iter()
        // 現在時刻より後に開始できるタスクから
        .filter(|task| task.start > current_time)
        // もっと思早く終わるタスクを選択
        .min_by_key(|task| task.end)
}
