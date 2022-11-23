pub mod my_bench{
    use crate::a_data::data;
    use std::{fmt, ffi::c_uint};
    use esp_idf_sys;

    pub struct timer{
        tStart          : i64,
        cStart          : c_uint,
        started         : bool,
        pub tDuration   : i64,
        pub cDuration   : u32,
    }

    impl timer{
        pub fn init() -> timer{
            timer {
                tStart      : 0,
                cStart      : 0,
                started     : false,
                tDuration   : 0,
                cDuration   : 0,
            }
        }
        
        pub fn start(&mut self){
            self.started = true;
            unsafe{
                self.tStart = esp_idf_sys::esp_timer_get_time();
                self.cStart = esp_idf_sys::xthal_get_ccount();
            }
        }
        
        pub fn stop(&mut self){
            unsafe{
                self.tDuration = esp_idf_sys::esp_timer_get_time() - self.tStart;
                self.cDuration = esp_idf_sys::xthal_get_ccount() - self.cStart;
            }
            if !self.started{
                println!("Timer was not started duration is invalid");
            }
            self.started = false;
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
                            println!("|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{}", data::LANG_NAME, self.testName, CPU_freq, iters+1, data_len, timer.cDuration, timer.tDuration, rez);
                        }
                        printType::CSV =>{
                            println!("{:<10};{:<10};{:<10};{:<10};{:<10};{:<10};{:<10};{}", data::LANG_NAME, self.testName, CPU_freq, iters+1, data_len, timer.cDuration, timer.tDuration, rez);
                        }
                        printType::Matlab => todo!(),
                    }
                }
            }
        }
    }
}

