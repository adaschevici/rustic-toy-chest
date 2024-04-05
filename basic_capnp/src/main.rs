use capnp::message::Builder;
use capnp::message::ReaderOptions;
use capnp::serialize;

pub mod person_capnp;

fn main() {
    let mut message = Builder::new_default();

    let mut person = message.init_root::<person_capnp::person::Builder>();
    person.set_name("John");
    person.set_age(23);

    let data = serialize::write_message_to_words(&message);
    println!("{:?}", data);

    let reader = serialize::read_message(data.as_slice(), ReaderOptions::new()).unwrap();

    let person = reader.get_root::<person_capnp::person::Reader>().unwrap();
    let name = person.get_name().unwrap();
    println!("Name: {:?}", name);
}
