use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;

#[derive(Clone)]
pub(crate) struct ClickListeners {
    id: Rc<RefCell<Vec<Box<dyn FnMut(Option<String>)>>>>,
}

impl ClickListeners {
    pub(crate) fn new() -> ClickListeners {
        ClickListeners { id: Rc::new(RefCell::new(Vec::new())) }
    }

    pub(crate) fn id(&self) -> Rc<RefCell<Vec<Box<dyn FnMut(Option<String>)>>>> {
        // Здесь извлекаем неизменяемую ссылку из RefCell
        self.id.clone()
    }

    pub(crate) fn add_listener(&mut self, handler: Box<dyn FnMut(Option<String>)>) {
        // Здесь изменяем содержимое через RefCell
        self.id.borrow_mut().push(handler);
    }
}
