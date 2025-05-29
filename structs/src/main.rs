#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * ( self.width + self.height )
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            return true
        } 
        return false 
    }
}


fn main() {
    let rec = Rectangle{
        height: 10,
        width: 5
    };

    let rec2 = Rectangle{
        height: 20,
        width: 12
    };

    println!("area: {}", rec.area());
    println!("peri: {}", rec.perimeter());
    println!("rec:  {rec:?}");
    println!("can rec1 hold rec2? {}", rec.can_hold(&rec2));
    println!("can rec2 hold rec1? {}", rec2.can_hold(&rec));
}

