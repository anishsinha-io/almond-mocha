use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationContainer<T> {
    pub items: Vec<T>,
    pub done: bool,
}

impl<T> PaginationContainer<T> {
    pub fn new(mut items: Vec<T>, limit: i64) -> Self {
        let len = items.len();
        let done = len < (limit as usize + 1);

        if !done {
            items.pop();
        };

        Self { items, done }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::util;

    use super::*;

    #[test]
    pub fn test_pagination() {
        util::test_util::init();

        let jennys_reading_list = vec![
            ("The Westing Game", "Ellen Raskin"),
            ("To Kill a Mockingbird", "Harper Lee"),
            ("Lord of the Flies", "William Golding"),
            ("Diary of Anne Frank", "Anne Frank"),
            ("A Separate Peace", "John Knowles"),
            ("The Hobbit", "J.R.R. Tolkien"),
            ("The Age of Innocence", "Edith Wharton"),
            ("Catcher in the Rye", "J.D. Salinger"),
            ("A Raisin in the Sun", "Lorraine Hansberry"),
            ("The Count of Monte Cristo", "Alexander Dumas"),
        ];

        let mut container = PaginationContainer::new(jennys_reading_list.clone(), 9);
        assert!(!container.done);

        container = PaginationContainer::new(jennys_reading_list.clone(), 10);
        assert!(container.done);

        container = PaginationContainer::new(jennys_reading_list.clone(), 11);
        assert!(container.done);
    }
}
