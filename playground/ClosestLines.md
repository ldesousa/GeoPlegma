
x,y - reals between 0 and 1

d - fixed power of three

n,m - unknown, but integers between 0 and d

i - numerator which over d equates to x
j - numerator which over d equates to y

# Odd levels

### relevant lines for i
n = floor(x/d)
m = ceiling(x/d)

p = round(y/d)

v - p < 3 (level)
p - w < 3 (level)

col_start = n % level
num_hops = y // level

### relevant lines for j
v = col_start + num_hops * level
w = col_start + (num_hops + 1) * level


# Even levels

### relevant lines for i
n = floor(x/d)
m = ceiling(x/d)

### relevant lines for j
 







|i/d - n/d| < 1



