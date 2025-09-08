#[macro_export]
macro_rules! handle_apply_event {
    (
        $fn_name:ident,
        $packet_id:expr,
        $handler:ident,
        $registry_type:ty,
        $packet_data_type:ty,
        $target_type:ty,
        $get_target_fn:ident,
    ) => {
        pub fn $fn_name<F>(&mut self, user_callback: F) -> &mut Self 
        where
            $packet_data_type: Parse + ProvideTargetKey + ApplyEvent<$target_type>,
            F: Fn(&$target_type) + Send + 'static
        {
            self.$handler.insert($packet_id, Box::new(move |registries, raw_bytes| {
                    
                // parse data
                let mut reader = Cursor::new(raw_bytes);
                let mut packet_data = <$packet_data_type>::parse(&mut reader).unwrap(); // temp shit

                // find registry
                if let Some(registry) = registries.get_mut(&TypeId::of::<$registry_type>())
                    .and_then(|any| any.downcast_mut::<$registry_type>()) {
                        if let Some(mut target) = registry.$get_target_fn(packet_data.key()) {
                                
                            // apply new info
                            packet_data.apply(&mut target);

                            // user callback
                            (user_callback)(&mut target)
                        }
                    }
            }));
            self
        }
    };
}

#[macro_export]
macro_rules! handle_spawn_event {
    (
        $fn_name:ident,
        $packet_id:expr,
        $handler:ident,
        $registry_type:ty,
        $packet_data_type:ty,
        $target_type:ty,
        $get_target_fn:ident,
    ) => {
        pub fn $fn_name<F>(&mut self, user_callback: F) -> &mut Self
        where
            $packet_data_type: Parse + ProvideTargetKey + SpawnEvent<$registry_type>,
            F: Fn(&$target_type) + Send + 'static
        {
            self.$handler.insert($packet_id, Box::new(move |registries, raw_bytes| {

                let mut reader = Cursor::new(raw_bytes);
                let mut packet_data = <$packet_data_type>::parse(&mut reader).unwrap(); // temp

                if let Some(mut registry) = registries.get_mut(&TypeId::of::<$registry_type>())
                    .and_then(|any| any.downcast_mut::<$registry_type>()) {
                        packet_data.spawn(&mut registry); 

                        if let Some(mut target) = registry.$get_target_fn(packet_data.key()) {
                            user_callback(&mut target);
                        } 
                    }
            }));
            self
        }
    };
}
// shit shit shit shit shit temp
#[macro_export]
macro_rules! handle_with_reply_event {
    (
        $fn_name:ident,
        $packet_id:expr,
        $handler:ident,
        $packet_data_type:ty, 
        $reply_packet_builder:ty, 
    ) => {
        pub fn $fn_name<F>(&mut self, user_callback: F) -> &mut Self
        where
            $packet_data_type: Parse + WithReply,
            <$packet_data_type as WithReply>::Reply: Sized, 
            $reply_packet_builder: DataBuilder<Data = <$packet_data_type as WithReply>::Reply>,
            F: Fn(&$packet_data_type) + Send + 'static
        {
            let sender = self.sender.clone();

            self.$handler.insert($packet_id, Box::new(move |_registries, raw_bytes| {

                let mut reader = Cursor::new(raw_bytes);
                let packet_data = <$packet_data_type>::parse(&mut reader).unwrap(); // temp
                let reply_data = packet_data.with_reply();
                let sender_clone = sender.clone();
                if let Ok(mut reply_bytes) = <$reply_packet_builder>::build(reply_data) {
                    let packet = encode::encode(&mut reply_bytes, 256).unwrap(); // absolute shit hardcode, for test temp
                    tokio::spawn(async move {
                    if let Err(e) = sender_clone.send(packet).await {
                            eprintln!("Failed, {}", e);
                        }
                    });
                } 
                (user_callback)(&packet_data);
            }));
            self
        }
    };
}

#[macro_export]
macro_rules! handle_remove_event {
    (
        $fn_name:ident,
        $packet_id:expr,
        $handler:ident,
        $registry_type:ty,
        $packet_data_type:ty,
    ) => {
        pub fn $fn_name<F>(&mut self, user_callback: F) -> &mut Self 
        where
            $packet_data_type: Parse + $crate::packets::types::RemoveEvent<$registry_type>,
            F: Fn(&$packet_data_type) + Send + 'static
        {
            self.$handler.insert($packet_id, Box::new(move |registries, raw_bytes| {

                let mut reader = Cursor::new(raw_bytes);
                let mut packet_data = <$packet_data_type>::parse(&mut reader).unwrap(); // temp shit

                if let Some(registry) = registries.get_mut(&TypeId::of::<$registry_type>())
                    .and_then(|any| any.downcast_mut::<$registry_type>()) {
                        packet_data.remove(registry);

                        (user_callback)(&packet_data);
                    }
            }));
            self
        }
    };
}

// Maybe hardcode InternalStorage???
#[macro_export]
macro_rules! handle_stateful_event {
    (
        $fn_name:ident,
        $packet_id:expr,
        $handler:ident,
        $registry_type:ty,
        $packet_data_type:ty,
    ) => {
        pub fn $fn_name<F>(&mut self, user_callback: F) -> &mut Self 
        where
            $packet_data_type: Parse + ApplyEvent<$registry_type>,
            F: Fn() + Send + 'static
        {
            self.$handler.insert($packet_id, Box::new(move |registries, raw_bytes| {
                println!("{:?}", raw_bytes);
                // parse data
                let mut reader = Cursor::new(raw_bytes);
                let mut packet_data = <$packet_data_type>::parse(&mut reader).unwrap(); // temp shit

                // find registry
                if let Some(mut registry) = registries.get_mut(&TypeId::of::<$registry_type>())
                    .and_then(|any| any.downcast_mut::<$registry_type>()) {
                            // apply
                            packet_data.apply(&mut registry);

                            // user callback
                            (user_callback)()
                    }
            }));
            self
        }
    };
}