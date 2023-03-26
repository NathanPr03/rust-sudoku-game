pub struct UserInputCommand {
    x_coordinate: usize,
    y_coordinate: usize,
    new_value: usize,
    previous_value: usize
}

impl UserInputCommand {
    pub fn new(x: usize, y: usize, value: usize) -> UserInputCommand
    {
        return UserInputCommand
        {
            x_coordinate: x,
            y_coordinate: y,
            new_value: value,
            previous_value: 0
        }
    }

    pub fn execute()
    {

    }
}