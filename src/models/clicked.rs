use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;

#[derive(Clone)]
pub(crate) struct Clicked {
    id: Rc<RefCell<Vec<Box<dyn FnMut(Option<String>)>>>>,
}

impl Clicked {
    pub(crate) fn new() -> Clicked {
        Clicked { id: Rc::new(RefCell::new(Vec::new())) }
    }

    pub(crate) fn id(&self) -> Rc<RefCell<Vec<Box<dyn FnMut(Option<String>)>>>> {
        // Здесь извлекаем неизменяемую ссылку из RefCell
        self.id.clone()
    }

    pub(crate) fn push(&mut self, handler: Box<dyn FnMut(Option<String>)>) {
        // Здесь изменяем содержимое через RefCell
        self.id.borrow_mut().push(handler);
    }
}
