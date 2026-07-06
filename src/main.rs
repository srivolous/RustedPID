struct PID{
    kp: f64,
    ki: f64,
    kd: f64
    i:f64,
    p_err:f64,
}
impl PID{
    fn new(kp:f64,ki:f64,kd:f64)->Self{
        Self{
            kp,ki,kd,i:0.0,p_err:0.0
        }
    }
    fn calculate(&mut self, setzeug:f64, reading:f64,dt:f64)->f64{
        let error = setzeug - reading;
        let p = self.kp*error;
        self.i +=error*dt;
        let i_o = self.ki*self.i;
        let d = (error-self.p_err)/dt;
        let d_o = self.kd*derivative;
        self.p_err = err;
        p + i_o + d_o
    }
}
fn main() {
    println!("Hello world!");
}
