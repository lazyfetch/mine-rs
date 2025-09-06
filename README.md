mine-rs

`// temp` comment means that I need to check it several times and correct it

its test project so i dont will continue to support it or implement all features of minecraft protocol. I focus on the architecture of the project, its main workflow, and that's it :)

todo: 
1. make realization decode/encode.rs +
2. make handle.rs realization, use async (tokio) write for tcpstream and create send channel for infinity producers.
3. make simple example realization for player controller
4. FINAL BOSS. make full pipeline FROM handshake TO play stages (make internal master_handlers hashmap which contains closure's and packet_id, but dont forget about states, so make 2 master_handlers and check stages every time)
5. Maybe u need to create client_builder.player.with_N().with_N()... so do it 
6. Give to user handle and client struct's, handle for send packets, like, player_controller, and something else, client... for client.run which will be run play stage read thread, i think... 