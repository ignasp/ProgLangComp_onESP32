pub mod my_bench{
    use crate::a_data::data;
    use std::fmt;

    pub struct timer{
        tStart          : std::time::Instant,
        started         : bool,
        pub tDuration   : std::time::Duration,
        pub cDuration   : u32,
    }
    
    impl timer{
        pub fn init() -> timer{
            timer {
                tStart      : std::time::Instant::now(),
                started     : false,
                tDuration   : std::time::Duration::ZERO,
                cDuration   : 0,
            }
        }
        
        pub fn start(&mut self){
            self.started = true;
            self.tStart = std::time::Instant::now();
        }
        
        pub fn stop(&mut self){
            self.tDuration = self.tStart.elapsed();
            if !self.started{
                println!("Timer was not started duration is invalid");
            }
            self.started = false;
            self.cDuration = ((self.tDuration.as_nanos() as f32)/1000.0 * 160.0) as u32;
        }
    }
    
    //-------------------------------------------------------------------------
    
    pub enum rezVerifcation {
        Match,
        NoMatch,
        NoStd,
    }

    impl fmt::Display for rezVerifcation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           match *self {
               rezVerifcation::Match => write!(f, "OK"),
               rezVerifcation::NoMatch => write!(f, "Err"),
               rezVerifcation::NoStd => write!(f, "NoStd"),
           }
        }
    }
    
    //------------------------------------------------------------------------
    
    pub enum printType {
        Readable,
        Matlab,
        CSV,
    }
    
    pub fn print_header(ptype : printType){
        match ptype {
            printType::Readable => {
                println!("|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{}", "Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez");
            }
            printType::CSV =>{
                println!("{:<10};{:<10};{:<10};{:<10};{:<10};{:<10};{:<10};{}", "Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez");
            }
            printType::Matlab => todo!(),
        }
    }

    //-------------------------------------------------------------------------
    
    pub struct tester{
        pub testName : &'static str,
        pub testLengths : Vec<usize>,
        pub nIterations : u32,
        pub printType : printType,
        pub runFn : fn(usize, &mut timer) -> rezVerifcation,
    }
    
    impl tester{
        pub fn run(&mut self){
            let CPU_freq:u32;
            unsafe{
                CPU_freq = esp_idf_sys::ets_get_cpu_frequency();
            }
            let mut timer = timer::init();
            for i in 0..self.testLengths.len(){
                let data_len = self.testLengths[i];
                let _rez = (self.runFn)(data_len, &mut timer);
                for iters in 0..self.nIterations {
                    let rez = (self.runFn)(data_len, &mut timer);
                    match self.printType {
                        printType::Readable => {
                            println!("|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{}", data::LANG_NAME, self.testName, CPU_freq, iters+1, data_len, timer.cDuration, timer.tDuration.as_micros(), rez);
                        }
                        printType::CSV =>{
                            println!("{:<10};{:<10};{:<10};{:<10};{:<10};{:<10};{:<10};{}", data::LANG_NAME, self.testName, CPU_freq, iters+1, data_len, timer.cDuration, timer.tDuration.as_micros(), rez);
                        }
                        printType::Matlab => todo!(),
                    }
                }
            }
        }
    }
}

