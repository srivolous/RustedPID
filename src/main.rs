use chrono::{DateTime,Local};

struct PID{
    kp: f64,
    ki: f64,
    kd: f64,
    i:f64,
    p_err:f64,
    p_time:DateTime<Local>,
}

impl PID{
    fn new(kp:f64,ki:f64,kd:f64)->Self{
        Self{
            kp,ki,kd,i:0.0,p_err:0.0,p_time:Local::now(),
        }
    }
    fn calculate(&mut self, setzeug:f64, reading:f64)->f64{
        let error = setzeug - reading;
        let dtt = Local::now() - self.p_time;
        let dt = dtt.num_milliseconds() as f64/1000.0;
        let p = self.kp*error;
        self.i +=error*dt;
        let i_o = self.ki*self.i;
        let d = (error-self.p_err)/dt;
        let d_o = self.kd*d;
        self.p_err = error;
        self.p_time = Local::now();
        p + i_o + d_o
    }
}
fn main() {
    //let aa = Local::now();
    let mut P = 0.1;
    let mut I = P/10;
    let mut D = 0.0;
    let mut altcontrol = PID::new(&P,&I,&D);
    let mut rollcontrol = PID::new(&P,&I,&D)
    //std::thread::sleep(std::time::Duration::from_millis(10));
    let ree = n.calculate(2.0,5.0);
   // let a = Local::now();
}
