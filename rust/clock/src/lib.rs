#[derive(std::cmp::PartialEq)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = hours % 24; // get hours
        h = if h < 0 { 24 + h } else { h }; // if hours < 0 update.
        let hm = minutes / 60 as i32; // get full hours from minutes, could be < 0
        h = (h + hm) % 24;
        h = if h < 0 { 24 + h } else { h };
        let mut m = minutes % 60; // get rest of minutes < 60 min.
        m = if m < 0 {
            h = if h == 0 { 24 - 1 } else { h - 1 }; // because we are going backwards from hour
            60 + m // get rest of minutes
        } else {
            m
        };
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // get full hours and reminder of minutes from minutes
        // norm for normilized.
        let mut h_norm = minutes / 60 as i32; // get full hours from supplied minutes
        h_norm = h_norm % 24; // remove days overflow
        h_norm = if self.hours == 0 {
            24 + h_norm
        } else {
            self.hours + h_norm
        };
        h_norm = h_norm % 24; // remove days overflow

        let mut m_norm = minutes % 60; // get minutes remaning always less 60

        m_norm = self.minutes + m_norm; // combine minutes
                                        // we can overflow for negative -40 + - 30 = -70
        if m_norm < -60 {
            let tmp_h = m_norm / 60 as i32;
            m_norm = m_norm % 60;
            h_norm = if h_norm == 0 {
                24 + tmp_h
            } else {
                h_norm + tmp_h
            };
            h_norm = h_norm % 24; // remove days overflow
        }

        // after minutes combination m_norm must be less abs(60)
        if m_norm < 0 {
            // if still < 0
            h_norm = if h_norm == 0 { 23 } else { h_norm - 1 }; // because we are going backwards from hour
            m_norm = 60 + m_norm
        } else {
            let tmp_h = m_norm / 60 as i32;
            h_norm = h_norm + tmp_h;
            h_norm = h_norm % 24; // remove days overflow
            m_norm = m_norm % 60;
        }

        Clock {
            hours: h_norm,
            minutes: m_norm,
        }
    }
}

fn zero_pad(n: i32) -> String {
    if n >= 0 && n < 10 {
        format!("0{}", n)
    } else {
        format!("{}", n)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", zero_pad(self.hours), zero_pad(self.minutes))
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Clock{{ hours:{}, minutes: {}}}",
            zero_pad(self.hours),
            zero_pad(self.minutes)
        )
    }
}
