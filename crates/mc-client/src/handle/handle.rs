pub struct Handle {
    // here i need sender channel 
    // maybe link on registry or client or something else
    // maybe stream.write 
}

impl Handle {
    // here i create tokio thread
    // and make channel, so pipeline look like this
    // user -> player_controller/anything_alse.some_method(x,y,z) -> channel -> handle -> stream.write()
    // some_method(x,y,z) its like entity_handler_registry, so hes assembly serverbound packet and write it
    // he's like a node. 
    // in turn. Also need to think about make .some_method() more smart, or make like primitive
    // and dont think anything. Mean make add some temp data for client-side predict in .some_method()
    // if it'll useful ofc, i dont you for now.
    pub fn new() -> Handle {
        Handle {  }  
    }
}