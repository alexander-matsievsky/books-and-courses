#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    pub fn new(id: u64) -> CubeSat {
        CubeSat {
            id,
            mailbox: Mailbox { messages: vec![] },
        }
    }

    pub fn new_message(&self, content: &str) -> Message {
        Message {
            content: content.to_string(),
            to: self.id,
        }
    }

    pub fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }

    pub fn recv2(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox { messages: vec![] }
    }

    pub fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    content: String,
    to: u64,
}

impl Message {
    pub fn new(content: &str) -> Message {
        Message {
            content: content.to_string(),
            to: 0,
        }
    }

    pub fn new_to(to: u64, content: String) -> Message {
        Message { content, to }
    }
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

impl GroundStation {
    pub fn new() -> GroundStation {
        GroundStation { radio_freq: 0.0 }
    }

    pub fn new_radio_freq(radio_freq: f64) -> GroundStation {
        GroundStation { radio_freq }
    }

    pub fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }

    pub fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }

    pub fn send2(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

fn check_status(sat_id: CubeSat) -> (CubeSat, StatusMessage) {
    (sat_id, StatusMessage::Ok)
}

fn check_status_id(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::chapter_4_lifetimes_ownership_and_borrowing::*;

    #[test]
    fn main1() {
        let sat_a = 0;
        let sat_b = 1;
        let sat_c = 2;

        let a_status = check_status_id(sat_a);
        let b_status = check_status_id(sat_b);
        let c_status = check_status_id(sat_c);
        println!("a: {:?} b: {:?} c: {:?} ", a_status, b_status, c_status);

        let a_status = check_status_id(sat_a);
        let b_status = check_status_id(sat_b);
        let c_status = check_status_id(sat_c);
        println!("a: {:?} b: {:?} c: {:?} ", a_status, b_status, c_status);
    }

    #[test]
    fn main2() {
        let sat_a = CubeSat::new(0);
        let sat_b = CubeSat::new(1);
        let sat_c = CubeSat::new(2);

        let (sat_a, a_status) = check_status(sat_a);
        let (sat_b, b_status) = check_status(sat_b);
        let (sat_c, c_status) = check_status(sat_c);
        println!("a: {:?} b: {:?} c: {:?} ", a_status, b_status, c_status);

        let (_sat_a, a_status) = check_status(sat_a);
        let (_sat_b, b_status) = check_status(sat_b);
        let (_sat_c, c_status) = check_status(sat_c);
        println!("a: {:?} b: {:?} c: {:?} ", a_status, b_status, c_status);
    }

    #[test]
    fn main3() {
        let base = GroundStation::new();
        let mut sat_a = CubeSat::new(0);

        println!("t0: {:?}", sat_a);
        base.send(&mut sat_a, Message::new("hello, world!"));
        println!("t1: {:?}", sat_a);
        let msg = sat_a.recv();
        println!("t2: {:?}", sat_a);
        println!("msg: {:?}", msg);
    }

    #[test]
    fn main4() {
        let base = GroundStation::new();
        for sat_id in fetch_sat_ids() {
            let mut sat = base.connect(sat_id);
            base.send(&mut sat, Message::new("hello"));
            println!("{:?}", sat);
        }
    }

    #[test]
    fn main5() {
        let mut mail = Mailbox::new();
        let base = GroundStation::new();
        for sat_id in fetch_sat_ids() {
            let sat = base.connect(sat_id);
            let msg = sat.new_message("hello");
            base.send2(&mut mail, msg)
        }
        for sat_id in fetch_sat_ids() {
            let sat = base.connect(sat_id);
            let msg = sat.recv2(&mut mail);
            println!("{:?}: {:?}", sat, msg);
        }
    }

    #[test]
    fn main6() {
        let base = Rc::new(RefCell::new(GroundStation::new_radio_freq(87.65)));
        println!("base: {:?}", base);
        {
            let mut base_2 = base.borrow_mut();
            base_2.radio_freq -= 12.34;
            println!("base_2: {:?}", base);
        }
        println!("base: {:?}", base);

        let mut base_3 = base.borrow_mut();
        base_3.radio_freq -= 43.21;
        println!("base_3: {:?}", base_3);
        println!("base: {:?}", base);
    }
}
