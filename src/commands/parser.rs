use serde::{Serialize, Deserialize} 


[#derive(Serialize,Deserialize)]
struct Payload {
    id: String, 
    price: u16,
}

