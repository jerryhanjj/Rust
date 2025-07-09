trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}

impl Park for Car {
    fn park(&self) {
        println!("Parking Car...");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn upload(&self) {
        println!("uploading truck.")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("Parking Truck...");
    }
}

impl Paint for Truck {
    fn paint(&self, color: String) {
        println!("This is truck paint, object: {}", color);
    }
}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting House with color: {}", color);
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    };
    
    let house = House {};
    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    paint_vehicle_red(&car);

    println!("\n--- 动态分发演示 ---");
    demonstrate_dynamic_dispatch();
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

fn paint_red_2(object: &impl Paint) {
    object.paint("red".to_owned());
}

// 如果有多个 trait 需要绑定，要求 T 同时实现了 Paint 和 Park
fn paint_vehicle_red<T>(object: &T) where T: Paint + Park {
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint{
    House {}
}

// 动态分发版本 - 使用 trait 对象
fn paint_red_dynamic(object: &dyn Paint) {
    object.paint("red".to_owned());
}

// 返回 trait 对象的函数
fn create_paintable_objects() -> Vec<Box<dyn Paint>> {
    vec![
        Box::new(Car {
            info: VehicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995
            }
        }),
        Box::new(House {}),
        Box::new(Truck {
            info: VehicleInfo {
                make: "Ford".to_owned(),
                model: "F-150".to_owned(),
                year: 2020
            }
        })
    ]
}

// 演示动态分发的使用
fn demonstrate_dynamic_dispatch() {
    let objects = create_paintable_objects();
    
    // 运行时才知道具体调用哪个类型的 paint 方法
    for object in objects.iter() {
        paint_red_dynamic(object.as_ref());
    }
}