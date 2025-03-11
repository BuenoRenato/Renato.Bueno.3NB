struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> Queue<T> {
    // Cria uma nova fila vazia
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    // Insere um elemento no final da fila
    pub fn enqueue(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem,
            next: None,
        });

        let new_tail_ptr: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = new_tail_ptr;
        self.len += 1;
    }

    // Remove e retorna o elemento da frente da fila
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            self.len -= 1;
            head.elem
        })
    }

    // Retorna uma referência ao elemento da frente da fila sem removê-lo
    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }

    // Retorna o número atual de elementos na fila
    pub fn len(&self) -> usize {
        self.len
    }
}