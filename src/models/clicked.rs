use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct ClickListeners {
    id: Rc<RefCell<Vec<Box<dyn FnMut(Option<String>)>>>>,
}

impl ClickListeners {
    pub(crate) fn new() -> ClickListeners {
        ClickListeners { id: Rc::new(RefCell::new(Vec::new())) }
    }

    pub(crate) fn id(&self) -> Rc<RefCell<Vec<Box<dyn FnMut(Option<String>)>>>> {
        self.id.clone()
    }

    pub(crate) fn add_listener(&mut self, handler: Box<dyn FnMut(Option<String>)>) {
        self.id.borrow_mut().push(handler);
    }
}
