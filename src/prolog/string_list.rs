use prolog::tabled_rc::*;

use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Index, RangeTo};

#[derive(PartialOrd, PartialEq, Ord, Eq)]
pub struct StringListWrapper(RefCell<String>);

impl Hash for StringListWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state)
    }
}

// cursor is ignored if the double_quotes flag is set to atom
#[derive(Clone)]
pub struct StringList {
    body: TabledRc<StringListWrapper>,
    cursor: usize, // use this to generate a chars() iterator on the fly,
                   // and skip over the first cursor chars. 
    expandable: bool
}

impl Hash for StringList {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.borrow().hash(state)
    }
}

impl PartialOrd for StringList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.body.cmp(&other.body))
    }
}

impl Ord for StringList {
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.body.cmp(&other.body)
    }
}

impl PartialEq for StringList {
    fn eq(&self, other: &Self) -> bool
    {
        self.body == other.body
    }
}

impl Eq for StringList {}

impl StringList {
    #[inline]
    pub fn new(s: String, expandable: bool, string_tbl: TabledData<StringListWrapper>) -> Self {
        let body = TabledRc::new(StringListWrapper(RefCell::new(s)), string_tbl);

        StringList {
            cursor: 0,
            body,
            expandable
        }
    }

    #[inline]
    pub fn tail(&self) -> Self {
        let mut new_string_list = self.clone();
        
        if let Some(c) = self.borrow()[self.cursor ..].chars().next() {        
            new_string_list.cursor += c.len_utf8();
        }
        
        new_string_list
    }

    #[inline]
    pub fn borrow(&self) -> Ref<String> {
        self.body.0.borrow()
    }
}
