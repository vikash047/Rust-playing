mod list {
    pub struct Tasks {
        pub item:String,
    }

    pub mod things_todo {
        pub fn add_activity(){}
        fn update_activity(){}
        fn marked_completed(){}
    }

    mod item_complted {
        fn remove_task(){}
        fn move_back_todo() {}
    }
}

fn lets_add_task() {
    let task = list::Tasks{item: String::From("Task")};
}