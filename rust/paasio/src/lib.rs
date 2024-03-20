use std::{
    io::{Read, Result, Write},
    cell::RefCell,
};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them


/**
 * For now RefCell sims like a valid choice. I can not fully understand what is going on in the tests
 * And what most (all) methods in lib must do. Must write test of my own. Get better understanding of what is 
 * happening in the tests and what I actually must implememnt.
 */

pub struct ReadStats<R> (RefCell<R>);

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self(RefCell::new(wrapped))
    }

    pub fn get_ref(&self) -> &R {
        unsafe {self.0.try_borrow_unguarded().unwrap()}
    }

    pub fn bytes_through(&self) -> usize {
        self.reads()
    }

    pub fn reads(&self) -> usize {
        let b: &mut Vec<u8> = &mut vec![];
        self.0.borrow_mut().read_to_end(b).unwrap_or(0)
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.borrow_mut().read(buf)
    }
}

pub struct WriteStats<W>(W);

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self(wrapped)
    }

    pub fn get_ref(&self) -> &W {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        unimplemented!()
    }

    pub fn writes(&self) -> usize {
        unimplemented!()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        unimplemented!("Collect statistics about this call writing {buf:?}")
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}
