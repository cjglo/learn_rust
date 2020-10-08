// Enum example with IP addresses

fn main()
{

    
    enum IpAddrType
    {
        V4,
        V6,
    }
    
    struct IpAddr // Could add this to record address, but actually there is a more efficient way
    {
        kind: IpAddrType,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrType::V4,
        address: String::from("127.0.0.1"),
    };

    



}