/**

    1. Adding todos and fix me section in your code makes task tracing and bug identification a lot easier.
    2. Improves collaboration a lot.

    to check todos:
    triple dot -> TODOS

    1. Bookmarks:  CTRL + F11
    2. check bookmarks:  alt + 2
*/
pub fn task1()
    {
        //TODO: Implement this method
        //FIXME: Error bug in line 36
    
    }

pub fn task2()
    {
        // Task 2: Print a welcome message with additional details
        let greeting = "Hello";
        let name = "User";
        let app_name = "RustApp";
    
        println!("{} {}, welcome to {}! We hope you enjoy using it.", greeting, name, app_name);
        println!("Feel free to explore the features we have.");
    }

pub fn task3()
    {
        // Task 3: Generate a simple report and print it
        let tasks_done = 5;
        let tasks_remaining = 2;
        let total_tasks = tasks_done + tasks_remaining;
    
        println!("Task Report:");
        println!("Total tasks: {}", total_tasks);
        println!("Tasks completed: {}", tasks_done);
        println!("Tasks remaining: {}", tasks_remaining);
        println!("Keep up the good work!");
    }