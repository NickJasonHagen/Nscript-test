
use crate::*;

pub struct Ntimer {

}

impl Ntimer {
    pub fn diff(timerhandle: i64) -> i64 {
        let getnow = Ntimer::init();
        let diff = getnow - timerhandle;
        return diff;
    }
    pub fn init() -> i64 {
        let time = chrono::Utc::now();
        let mut timerstring = String::from(&time.year().to_string());
        if &time.month() < &10 {
            timerstring = timerstring + "0" + &time.month().to_string();
        } else {
            timerstring = timerstring + &time.month().to_string();
        }
        // check day for 2 characters
        if &time.day() < &10 {
            timerstring = timerstring + "0" + &time.day().to_string();
        } else {
            timerstring = timerstring + &time.day().to_string();
        }
        // check hour for 2 characters
        if &time.hour() < &10 {
            timerstring = timerstring + "0" + &time.hour().to_string();
        } else {
            timerstring = timerstring + &time.hour().to_string();
        }
        // check minute for 2 characters
        if &time.minute() < &10 {
            timerstring = timerstring + "0" + &time.minute().to_string();
        } else {
            timerstring = timerstring + &time.minute().to_string();
        }
        // check second for 2 characters
        if &time.second() < &10 {
            timerstring = timerstring + "0" + &time.second().to_string();
        } else {
            timerstring = timerstring + &time.second().to_string();
        }
        // check milisecond for 3 characters
        if &time.timestamp_subsec_millis() < &100 {
            if &time.timestamp_subsec_millis() < &10 {
                timerstring = timerstring + "00" + &time.timestamp_subsec_millis().to_string();
            } else {
                timerstring = timerstring + "0" + &time.timestamp_subsec_millis().to_string();
            }
        } else {
            timerstring = timerstring + &time.timestamp_subsec_millis().to_string();
        }
        return timerstring.parse::<i64>().unwrap();
    }

    pub fn hours_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"3600000")).to_string() ;
    }
    pub fn minutes_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"60000")).to_string() ;
    }
    pub fn days_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"86400000")).to_string() ;
    }
    pub fn weeks_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"604800000")).to_string() ;
    }
    pub fn months_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"2629800000")).to_string() ;
    }
    pub fn years_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"31557600000")).to_string() ;
    }
}


