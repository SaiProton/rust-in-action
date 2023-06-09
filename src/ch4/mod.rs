use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

impl GroundStation {
    fn connect(sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

pub fn main() {
    let mut mail = Mailbox { messages: vec![] };

    // let base = GroundStation { radio_freq: 1.11 };

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        // let sat = GroundStation::connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        GroundStation::send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = GroundStation::connect(sat_id);

        let Some(msg) = sat.recv(&mut mail) else { panic!("No message!") };

        println!("{sat:?}: {0:?}", msg.content);
    }

    // Allows for the creation of multiple mutable references.
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base: {base:?}");

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2 {base_2:?}");
    }

    println!("base: {base:?}");

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {base:?}");
    println!("base_3: {base_3:?}");
}
