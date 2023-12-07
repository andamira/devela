# Benchmarks
The current algorithm used can be more than 1 order of magnitude more
efficient than a naive loop that sets the individual bits, as well as
much more consistant across the spectrum of bitsizes.

The following table shows the compared benchmark for `start=0`
and `end=BITS-1` measured on an i5-8350U CPU @ 1.70 Ghz:
```table
bits   current    naive
----   -------   --------
 128    3.7 ns   145.0 ns
  64    3.2 ns    38.2 ns
  32    3.2 ns     9.0 ns
  16    3.2 ns     7.0 ns
   8    3.1 ns     6.7 ns
```
