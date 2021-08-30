/*
Library file used for random number generation. 

This is mainly a port of the ran2() function, originally written in C, found in page 282 of the book
    `Numerical Recipes in C, Second Edition (1992)`; by Press, Teukolsky, Vetterling and Flannery.

A free, online version of this book can be found at the time of writing at `www.numerical.recipes`.
*/


const IM1: i32 = 2147483563;
const IM2: i32 = 2147483399;
const AM: f64 = 1.0 / IM1 as f64;
const IMM1: i32 = IM1 - 1;
const IA1: i32 = 40014;
const IA2: i32 = 40692;
const IQ1: i32 = 53668;
const IQ2: i32 = 52774;
const IR1: i32 = 12211;
const IR2: i32 = 3791;
const NTAB: i32 = 32;
const NDIV: i32 = 1 + IMM1 / NTAB;
const EPS: f64 = 1.2e-7;
const RNMX: f64 = 1.0 - EPS;

#[derive(Clone)]
pub struct Ran2Generator {
    idum: i32,
    idum2: i32,
    iy: i32,
    iv: [i32; NTAB as usize],
}

impl Ran2Generator {
    pub fn new(idum: i32) -> Ran2Generator {
        // Instantiate our parameters for ran2()
        let params: Ran2Generator = Ran2Generator {
            idum: idum,
            idum2: 123456789,
            iy: 0,
            iv: [0; NTAB as usize],
        };

        params
    }

    pub fn next(&mut self) -> f64 {        
        // Calculate the new random number
        let (idum, idum2, iy, iv, x) =
            ran2(self.idum, self.idum2, self.iy, self.iv);

        // Update self with the new parameters
        self.idum = idum;
        self.idum2 = idum2;
        self.iy = iy;
        self.iv = iv;

        // Return the random number
        x
    }
}

pub fn ran2(
    mut idum: i32,
    mut idum2: i32,
    mut iy: i32,
    mut iv: [i32; NTAB as usize],
) -> (i32, i32, i32, [i32; NTAB as usize], f64) {
    let mut j: i32;
    let mut k: i32;
    let temp: f64;

    if idum <= 0 {
        if -idum < 1 {
            idum = 1;
        } else {
            idum = -idum;
        }
        idum2 = idum;

        j = NTAB + 7;
        while j >= 0 {
            k = idum / IQ1;
            idum = IA1 * (idum - k * IQ1) - k * IR1;
            if idum < 0 {
                idum += IM1;
            }
            if j < NTAB {
                iv[j as usize] = idum;
            }
            j -= 1;
        }
        iy = iv[0];
    }
    k = idum / IQ1;
    idum = IA1 * (idum - k * IQ1) - k * IR1;
    if idum < 0 {
        idum += IM1
    }
    k = idum2 / IQ2;
    idum2 = IA2 * (idum2 - k * IQ2) - k * IR2;
    if idum2 < 0 {
        idum2 += IM2
    }
    j = iy / NDIV;
    iy = iv[j as usize] - idum2;
    iv[j as usize] = idum;
    if iy < 1 {
        iy += IMM1
    }

    temp = AM * f64::from(iy);
    if temp > RNMX {
        (idum, idum2, iy, iv, RNMX)
    } else {
        (idum, idum2, iy, iv, temp)
    }
}
