use core::ops::DerefMut;
use std::ops::Deref;

use rand::RngCore;

/// used to generate random booleans using a bit buffer
pub struct BufRand<R: RngCore> {
    /// this is a buffer of bits for random booleans
    bit_buf: u64,
    /// this indicates how many bits have been read from `bit_buf`.
    /// `bit_buf` is not simply checked against 0 to prevent slight bias to false
    shift_counter: u8,
    rand: R,
}

impl<R: RngCore> DerefMut for BufRand<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rand
    }
}

impl<R: RngCore> Deref for BufRand<R> {
    type Target = R;
    /// allows borrowing the randomizer this `BufRand` uses
    fn deref(&self) -> &Self::Target {
        &self.rand
    }
}

impl<R: RngCore> BufRand<R> {
    /// create a new `BufRand` which generates random booleans using the supplied rng
    pub fn new(rand: R) -> Self {
        BufRand {
            bit_buf: 0,
            shift_counter: 0xff, //not initialized with 0 to prevent bit_buf from being all 0
            rand,
        }
    }

    /// returns a random boolean from the buffer of this `BufRand`
    /// ```
    /// use buf_rand::BufRand;
    /// 
    /// let mut rand = BufRand::new(Box::new(rand::thread_rng()));
    /// rand.next_bool();
    /// ```
    pub fn next_bool(&mut self) -> bool {
        if self.shift_counter >= 64 {
            self.bit_buf = self.next_u64();
            self.shift_counter = 0;
        }
        let out = self.bit_buf % 2 == 0;
        self.bit_buf >>= 1;
        self.shift_counter += 1;
        out
    }

    /// randomizes the capitalization of a char
    /// this returns a `String` since some chars may result in multiple when the case is changed
    /// such as this
    /// ```
    /// //example of character turning into multiple
    /// assert_eq!('ß'.to_uppercase().to_string(), "SS");
    /// use buf_rand::BufRand;
    /// 
    /// let mut rand = BufRand::new(Box::new(rand::thread_rng()));
    /// rand.rand_char_case(&'E');
    /// ```
    pub fn rand_char_case(&mut self, c: &char) -> String {
        if self.next_bool() {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }
    }

    /// randomizes the capitalization of every character in the string
    /// ```
    /// use buf_rand::BufRand;
    /// 
    /// let mut rand = BufRand::new(Box::new(rand::thread_rng()));
    /// rand.rand_string_case("hello world!");
    /// ```
    pub fn rand_string_case(&mut self, s: &str) -> String {
        s.chars()
            .into_iter()
            .map(|c| self.rand_char_case(&c))
            .collect()
    }
}
