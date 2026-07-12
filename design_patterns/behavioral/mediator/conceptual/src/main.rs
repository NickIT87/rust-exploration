use std::cell::RefCell;
use std::rc::Rc;

trait Mediator {
    fn notify(&self, sender: &str, event: &str);
}

struct Component1 {
    mediator: Option<Rc<dyn Mediator>>,
}

impl Component1 {
    fn new() -> Self {
        Self { mediator: None }
    }

    fn do_a(&self) {
        println!("Component1 does A.");
        if let Some(mediator) = &self.mediator {
            mediator.notify("Component1", "A");
        }
    }

    fn do_b(&self) {
        println!("Component1 does B.");
        if let Some(mediator) = &self.mediator {
            mediator.notify("Component1", "B");
        }
    }
}

struct Component2 {
    mediator: Option<Rc<dyn Mediator>>,
}

impl Component2 {
    fn new() -> Self {
        Self { mediator: None }
    }

    fn do_c(&self) {
        println!("Component2 does C.");
        if let Some(mediator) = &self.mediator {
            mediator.notify("Component2", "C");
        }
    }

    fn do_d(&self) {
        println!("Component2 does D.");
        if let Some(mediator) = &self.mediator {
            mediator.notify("Component2", "D");
        }
    }
}

struct ConcreteMediator {
    component1: Rc<RefCell<Component1>>,
    component2: Rc<RefCell<Component2>>,
}

impl Mediator for ConcreteMediator {
    fn notify(&self, _sender: &str, event: &str) {
        match event {
            "A" => {
                println!("Mediator reacts on A:");
                self.component2.borrow().do_c();
            }
            "D" => {
                println!("Mediator reacts on D:");
                self.component1.borrow().do_b();
                self.component2.borrow().do_c();
            }
            _ => {}
        }
    }
}

fn main() {
    let c1 = Rc::new(RefCell::new(Component1::new()));
    let c2 = Rc::new(RefCell::new(Component2::new()));

    let mediator = Rc::new(ConcreteMediator {
        component1: Rc::clone(&c1),
        component2: Rc::clone(&c2),
    });

    c1.borrow_mut().mediator = Some(mediator.clone());
    c2.borrow_mut().mediator = Some(mediator.clone());

    println!("Client triggers A:");
    c1.borrow().do_a();

    println!();

    println!("Client triggers D:");
    c2.borrow().do_d();
}

// Client triggers A:
// Component1 does A.
// Mediator reacts on A:
// Component2 does C.

// Client triggers D:
// Component2 does D.
// Mediator reacts on D:
// Component1 does B.
// Component2 does C.