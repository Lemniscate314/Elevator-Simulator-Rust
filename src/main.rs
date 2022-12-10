use std::io;
trait TElevator 
{
    fn go_up(&self);
    fn go_down(&self);
    fn ask_passenger();
    fn select_floor(&self);
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
    destination_list:Vec<i32>



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
    fn select_floor(&self) {
        let is_valid_entry=true;
        loop
        {
            if !is_valid_entry
            {
                break;
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let floor: i32 = input.parse().unwrap_or(0);
            println!("You entered: {}", floor);

            match floor
            {
                f if f < self.min_floors => println!("Error. You have entered out of range floor. Valid [1-20]"),
                f if f > self.max_floors => println!("Error. You have entered out of range floor. Valid [1-20]"),
                f if f == self.current_floor => println!("You already in that floor"),
                _ => {}
            };
        }


    }
    fn ask_passenger()
    {

    }
}
fn start_simulation()
{

}
fn main() {
 
    let elevator = Elevator 
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
    };
      println!("On teste {}",elevator.current_floor);
      elevator.select_floor();
    
   
}
