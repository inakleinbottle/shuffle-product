type Degree = usize;


struct Word(Vec<u16>);

impl AsRef<[u16]> for Word {
    fn as_ref(&self) -> &[u16] {
        &self.0
    }
}

impl AsMut<[u16]> for Word {
    fn as_mut(&mut self) -> &mut [u16] {
        &mut self.0
    }
}


#[derive(Copy, Clone)]
struct WordMask(usize, usize, Degree);

impl WordMask {
    const fn new(raw: usize, word_length: usize, width: Degree) -> WordMask {
        WordMask(raw, word_length, width)
    }

    #[inline]
    const fn apply(&self, word: &impl AsRef<[u16]>) -> [usize; 2] {
        let letters = word.as_ref();
        assert!(letters.len() >= self.1);
        let mut m = self.0;

        let mut result: [usize; 2] = [0; 2];
        for i in 0..self.1 {
            if m & 1 == 0 {
                result[0] *= self.2;
                result[0] += letters[i];
            } else {
                result[1] *= self.2;
                result[1] += letters[i];
            }
            m >>= 1;
        }

        result
    }
}



struct WordMaskIterator {
    current: usize,
    end: usize,
    word_length: Degree,
    width: Degree,
}

impl WordMaskIterator {
    const fn new(width: Degree, word_length: Degree) -> WordMaskIterator {
        WordMaskIterator {
            current: 0usize,
            end: 1usize << (word_length + 1),
            word_length,
            width,
        }
    }
}

impl Iterator for WordMaskIterator {
    type Item = WordMask;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let mask = WordMask::new(self.current, self.word_length, self.width);
            self.current += 1;
            Some(mask)
        } else {
            None
        }
    }
}


fn inner_loop_slow(
    output: &mut f64,
    input_x: &[f64],
    input_y: &[f64],
    word: &Word,
    width: Degree,
    unfixed_mask_letters: Degree
) {
    for mask in WordMaskIterator::new(width, unfixed_mask_letters) {
        let [lidx, ridx] = mask.apply(word);

        *output += input_x[lidx]*input_y[ridx];
    }
}

fn out_loop_slow(

) {}




fn main() {
    println!("Hello, world!");
}
