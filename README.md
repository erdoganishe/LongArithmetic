# Long aripmetic

This project - expansion of project that mas made for Cryptography for developers course. (https://github.com/erdoganishe/BigMathRust)

## Functionality:

- BigNumber struct for unsigned long ints

- Convertation to string and back, to usize and back (usize can be converted to every unsigned int or int)

- Binary operations: inversion, xor, or, and, right-/left-shift;

- Comparation

- Basic aripmetic operations: addition, substraction, div and mod, multiplication and power.

 ### For multiplication implimented 3 algoritms:

- Basic multiplication: 
    Convert to usize and miltiply as usize, then convert to BigNumber: works for small values, using at Karatsuba algo

- Karatsuba multiplication:
    Using 3 "small number" multiplications for each 2 big number multiplications instead of 4, as result it uses $`O(N^{1,6})`$ instead $`O(N^2)`$ operations, but my algo is a shame and works too slow.

- Multiplication:
    Convert second value to string with radix 2.
    for each byte we double the result (result = result+result), and if byte was 1 add first value to result.
    $`O(n) \approx  k\cdot n\cdot ln(n) `$, *k* is constant so we can ignore it for $`O(n)`$, n is lenght of 2-radix string of second value;


### To run program:

To run program you must have rust lang installed, go to folder: 

        cd .\BigMathRust

and execute 
    
        cargo run

in cmd.
