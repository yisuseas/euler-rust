## For the 'sum_divisible_by()' function

Let the sum of numbers divisible by 3 up to 100 be:

$$ 3+6+9+\dots+99 = 3(1+2+3+\dots+33) $$

Note that 33 = 99 / 3 , but also (100 - 1) / 3 rounded down to the nearest integer (floor). i.e.

$$ int\left(\frac{100 - 1}{3}\right) = 33 $$

If we also note that the sum of a series of integers can be reduced this way:

$$ \sum_{i=1}^{n} i = \frac{n(n+1)}{2} $$

Then the sum of numbers divisible by a divisor __d__ up to a non-inclusive target __t__ be:

$$ p = int\left(\frac{t-1}{d}\right) $$
$$ sum = 3 \sum_{i=1}^{p} i = 3 \times \frac{p(p+1)}{2} $$
