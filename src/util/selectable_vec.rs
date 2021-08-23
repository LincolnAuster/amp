use std::ops::Deref;

/// Allow selection of *either* an index *or* a custom T.
pub enum Selection<T> {
    Index(usize),
    Custom(T),
}

/// A simple decorator around a Vec that allows a single element to be selected.
/// The selection can be incremented/decremented in single steps, and the
/// selected value wraps when moved beyond either edge of the set.
pub struct SelectableVec<T> {
    set: Vec<T>,
    selected: Selection<T>,
}

impl<T> SelectableVec<T> {
    pub fn new(set: Vec<T>) -> SelectableVec<T> {
        SelectableVec {
            set,
            selected: Selection::Index(0),
        }
    }

    pub fn new_custom(val: T) -> SelectableVec<T> {
        SelectableVec {
            set: Vec::new(),
            selected: Selection::Custom(val),
        }
    }

    pub fn selection(&self) -> Option<&T> {
        match self.selected {
            Selection::Index(i) => self.set.get(i),
            Selection::Custom(ref t) => Some(t),
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        match self.selected {
            Selection::Index(i) => Some(i),
            _ => None,
        }
    }

    /// Select the previous item, so long as one exists. Note that this is a
    /// noop if the selection is custom.
    pub fn select_previous(&mut self) {
        if let Selection::Index(i) = &mut self.selected {
            if *i > 0 {
                *i -= 1;
            } else {
                *i = self.set.len() - 1;
            }
        }
    }

    /// Select the previous item - noop if the item is custom.
    pub fn select_next(&mut self) {
        if let Selection::Index(i) = &mut self.selected {
            if *i < self.set.len() - 1 {
                *i += 1;
            } else {
                *i = 0;
            }
        }
    }

    pub fn select_custom(&mut self, custom: T) {
        self.selected = Selection::Custom(custom);
    }
}

impl<T> Deref for SelectableVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.set
    }
}

#[cfg(test)]
mod tests {
    use super::SelectableVec;

    #[test]
    fn selection_returns_none_when_the_set_is_empty() {
        let selectable_vec: SelectableVec<usize> = SelectableVec::new(Vec::new());
        assert!(selectable_vec.selection().is_none());
    }

    #[test]
    fn selection_returns_selected_element() {
        let mut selectable_vec: SelectableVec<usize> = SelectableVec::new(vec![0, 1, 2]);
        selectable_vec.select_next();
        assert_eq!(selectable_vec.selection(), Some(&1));
    }

    #[test]
    fn select_next_wraps_at_end_of_set() {
        let mut selectable_vec: SelectableVec<usize> = SelectableVec::new(vec![0, 1]);
        selectable_vec.select_next();
        selectable_vec.select_next();
        assert_eq!(selectable_vec.selection(), Some(&0));
    }

    #[test]
    fn select_previous_wraps_at_start_of_set() {
        let mut selectable_vec: SelectableVec<usize> = SelectableVec::new(vec![0, 1]);
        selectable_vec.select_previous();
        assert_eq!(selectable_vec.selection(), Some(&1));
    }
}
