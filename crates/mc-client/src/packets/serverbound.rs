// 0x00 handshaking
// protocol version - VarInt (772)
// Server address - string (255) 127.0.0.1 like u know
// Port - ushort 
// Intent - VarInt Enum 1. Status 2. Login 3. Transfer (2 for us)
// 0x00 login
// name - String(16)
// uuid - uuid (unused)
// 0x01 skip, cuz we dont use encryption for now
// 0x02 also
// 0x03 login acknowledged
// 0x00 configure. Well must next packets need to ping-pong meanings so server send some shit to client, client response
// its mean login_handler_registry or something else to boostrap our client, make all steps and success coming on Login stage

