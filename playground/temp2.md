0  
 
1   
  3
2
  4
3   6
  5
4   7
  6   9
      

Level 3                        column
-------                       start num
            9,0                   0   1
                                  2   0
            7,1                   1   1
        6,0     6,3               0   2
            5,2                   2   1
        4,1     4,5               1   2
    3,0     3,3     3,6           0   3
        2,2     2,5               2   2
    1,1     1,4     1,7           1   3
0,0     0,3     0,6     0,9       0   4


start of row: 0 to aperture^(level-1)
start of col: 1 to level-1

max num hops: (denom+1)%level


num_hops =  (denom+1) % level

for row in (1,int(denom+1)):
  col_start = row % level
  for col in (col_start,num_hops):

      ...
  if ((row+1) % level) = 0:
    num_hops = num_hops - 1
    num_hops = num_hops + 1
