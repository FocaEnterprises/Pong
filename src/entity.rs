trait Entity {
    fn get_x(&self) -> f64;
    fn set_x(&self, x: f64);
    
    fn get_y(&self) -> f64;
    fn set_y(&self, y: f64);

    fn get_width(&self) -> f64;
    fn set_width(&self, width: f64);
    
    fn get_height(&self) -> f64;
    fn set_height(&self, height: f64);
}