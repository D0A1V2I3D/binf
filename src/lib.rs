/*!
TODO documentation
*/

#[cfg(test)]
mod tests;

pub mod flag_utils;
pub mod macros;

///struct that holds data for a flag and useful functions
pub struct Flag {
    pub fvec: Vec<bool>,
}

impl Flag {
    /// add a flag value (if flag already exists returns an Err)
    pub fn add_flag(&mut self, flag: i32, value: bool) -> Result<(), String> {
        if self.exists(flag) {
            Err("Flag allredy exists".to_string())
        } else {
            self.set_flag(flag, value);
            Ok(())
        }
    }

    /// set a flag value regardless if it exists or not
    pub fn set_flag(&mut self, flag: i32, value: bool) {
        self.pad(flag).unwrap_or(());
        self.fvec[(flag as usize)] = value;
    }

    /// get a flags value if it doesn't exist returns err
    pub fn get_flag(&self, flag: i32) -> Result<bool, String> {
        if (self.fvec.len() as i32) < flag {
            Err("flag is outside of existing bitarray".to_string())
        } else {
            Ok(self.fvec[(flag as usize)])
        }
    }

    /// returns a vector with all enabled flags
    pub fn get_all_flags(&self) -> Vec<i32> {
        let mut tvec: Vec<i32> = Vec::new();
        for (i, value) in self.fvec.iter().enumerate() {
            if *value {
                tvec.push(i as i32);
            }
        }
        tvec
    }

    /// extends the Vec<bool> to set size (so that getflag wont err)
    pub fn pad(&mut self, len: i32) -> Result<(), i32> {
        if (self.fvec.len() as i32) >= len + 1 {
            Err(self.fvec.len() as i32)
        } else {
            while (self.fvec.len() as i32) != len + 1 {
                self.fvec.push(false)
            }
            Ok(())
        }
    }

    /// checks if a flag at index exists
    pub fn exists(&self, flag: i32) -> bool {
        !((self.fvec.len() as i32) < flag)
    }

    /// creates Flag from int
    pub fn new_from_existing(value: i32) -> Self {
        let mut tvec: Vec<bool> = Vec::new();
        let mut rest = value;
        while rest != 0 {
            tvec.push((rest % 2) != 0);
            rest = rest / 2;
        }
        Self { fvec: tvec }
    }

    /// returns flags
    pub fn get(&self) -> i32 {
        let base: i32 = 2;
        let mut num = 0;
        for (i, value) in self.fvec.iter().enumerate() {
            if *value {
                num += base.pow(i as u32);
            }
        }
        num
    }

    /// initializes a Flag with 0, use flag_new! macro to create with flags instead
    pub fn new() -> Self {
        Self { fvec: vec![] }
    }
}
