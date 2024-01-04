    #[derive(Debug)]
   pub enum IpAddress {
       V4(u8,u8,u8,u8),
        V6(String),
    }