# Benchmarks
The current algorithm used can be more than 1 order of magnitude more
efficient than a naive loop that sets the individual bits, as well as
much more consistant across the spectrum of bitsizes.

The following table shows the compared benchmark for `start=0`
and `end=BITS-1` measured on an i5-8350U CPU @ 1.70 Ghz:
```table
bits   current    naive
----   -------   --------
 128    2.8 ns   163.0 ns
  64    2.0 ns    21.6 ns
  32    2.4 ns    12.1 ns
  16    2.6 ns     7.6 ns
   8    2.9 ns     8.4 ns
```
