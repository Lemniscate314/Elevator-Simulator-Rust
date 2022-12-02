trait TElevator 
{
    fn go_up();
    fn go_down();
    fn ask_passenger();
    fn select_floor(id:i32);
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



}
impl TElevator for Elevator 
{
    fn go_up()
    {

    }
    fn go_down() {
        
    }
    fn ask_passenger() {
        
    }
    fn select_floor(id:i32) {
        
    }
}

fn main() {
    /* let mut elevator= Elevator 
    {
        list_of_floor : vec![1,2,3]
    }; */
}
