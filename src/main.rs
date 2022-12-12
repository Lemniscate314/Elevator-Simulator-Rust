use std::io;
use std::thread;
use std::time::Duration;
trait TElevator 
{
    fn go_up(&self);
    fn go_down(&self);
    fn ask_passenger(&self);
    fn select_floor(&self,id:i32);
}
struct Elevator 
{
    list_of_floor:Vec<i32>,
    is_door_open:bool,
    max_passengers:i32,
    min_passengers:i32,
    max_floors:i32,
    min_floors:i32,
    current_floor:i32,
    destination_floor:i32,
    destination_list:Vec<i32>,
    num_of_passengers:i32,
    passenger_floor:i32,
}
impl TElevator for Elevator 
{
    fn go_up(&self)
    {
        let next: i32 = self.current_floor + 1;
       println!("{}F | Going up ...",next);
       

    }
    fn go_down(&self) {
        let mut next: i32 = self.current_floor - 1;
        println!("{}F | Going up ...",next); 
    }
    fn select_floor(&mut self,id:i32)->i32{
        let mut is_valid_entry=false;
        loop
        {
            if is_valid_entry
            {
                break;
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let floor: i32 = input.as_str().parse().expect("Failed to parse input as integer");
            println!("You entered: {}", floor);
            match floor
            {
                f if f < self.min_floors => println!("Error. You have entered out of range floor. Valid [1-20]"),
                f if f > self.max_floors => println!("Error. You have entered out of range floor. Valid [1-20]"),
                f if f == self.current_floor => println!("You're already in that floor"),
                _ => {
                    if let Some(x) = self.destination_list.get_mut(floor-1) {
                        *x += 1;
                        break;
                    }
                }
            };
        }
        floor

    }
    fn ask_passenger(&mut self)
    {
        self.is_door_open = false;
        println!("Elevator opening ...");
        thread::sleep(Duration::new(2,0));
        self.is_door_open = true;
        println!("{} +F | How many passengers : ",self.current_floor);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.num_of_passengers = input.parse().unwrap_or(0);
        match self.num_of_passengers
        {
            nop if  nop < self.min_passengers || nop > self.max_passengers => {
                println!("Error. valid number of passengers [1-20]");
                self.ask_passenger()
            },
            _ => {
                for i in 0..=self.num_of_passengers -1
                {
                    let floor = self.select_floor(a);
                    assert!(self.list_of_floor.contains(&floor));
                }
            }
        }
        println!("Elevator is closing ...");

    }
}
fn start_simulation()
{

}
fn main() {
 
    let mut elevator = Elevator
    {
        current_floor:2,
        list_of_floor:Vec::new(),
        is_door_open: true,
        max_passengers: 20,
        min_passengers: 0,
        max_floors: 20,
        min_floors: 1,
        destination_floor: 1,
        destination_list:Vec::new(),
        num_of_passengers:3,
        passenger_floor:2,
    };
    let id = 2;
      elevator.select_floor(id);
    
   
}
