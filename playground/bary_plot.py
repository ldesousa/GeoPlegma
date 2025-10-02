from math import sqrt, pow
import matplotlib.pyplot as plt

def barycentric_to_cartesian(a, b, c, u, v, w):
    """
    Convert barycentric coordinates (u, v, w) relative to triangle vertices a, b, c
    into Cartesian coordinates.

    Parameters:
    a, b, c : tuples or lists
        Vertices of the triangle, each as (x, y).
    u, v, w : float
        Barycentric coordinates, satisfying u + v + w = 1.

    Returns:
    (x, y) : tuple
        Cartesian coordinates corresponding to barycentric coordinates.
    """

    # Extract vertices
    a1, a2 = a
    b1, b2 = b
    c1, c2 = c

    # Compute Cartesian coordinates
    x = u * a1 + v * b1 + w * c1
    y = u * a2 + v * b2 + w * c2

    return (x, y)


# Triangle vertices
a = (0, 0)
b = (100, 0)
c = (50, sqrt(pow(100,2) - pow(50,2)))

x_values = [a[0], b[0], c[0], a[0]]
y_values = [a[1], b[1], c[1], a[1]]
  
plt.plot(x_values, y_values, marker='o')  # Optional: add markers at points
plt.xlabel('X')
plt.ylabel('Y')
plt.title('Barycentric plots')
plt.grid(True)
plt.axis('equal')  # To keep aspect ratio equal

# Barycentric coordinates
u, v, w = (1.0/3), (1.0/3), (1.0/3)

cartesian_point = barycentric_to_cartesian(a, b, c, u, v, w)
plt.plot(cartesian_point[0], cartesian_point[1], marker='o')  # Optional: add markers at point

############# Test for Level 3 #############
aperture = 3
level = 3
denom = pow(aperture,level-1)
odd = 0

# loop columns
for top in range (0,int(denom)+1,aperture):
    # differenciate odd from even
    odd = odd % 2
    i_array = []
    j_step = 0
    j_array = []
    print("\n === top:%s ===" % (str(top)))
    # loop rows
    for i in range (0,top+1,2):
        i_array.append(i + odd)
        j_array.append(int(denom - top + j_step))
        j_step = j_step + 1
    
    # collect values from array and plot
    j_array = list(reversed(j_array))
    for x in range(0,len(i_array)):
        
        print("%s;%s" % (str(i_array[x]),str(j_array[x])))
        cartesian_point = barycentric_to_cartesian(
                          a, b, c, 
                          (i_array[x]/denom), 
                          (j_array[x]/denom), 
                          ((denom - i_array[x] - j_array[x])/denom))
        plt.plot(cartesian_point[0], cartesian_point[1], marker='o',
                 color="orange")
        cartesian_point = barycentric_to_cartesian(
                          a, b, c, 
                          (i_array[x]/denom), 
                          ((denom - i_array[x] - j_array[x])/denom),
                          (j_array[x]/denom))
        plt.plot(cartesian_point[0], cartesian_point[1], marker='o',
                 color="orange")
    
    odd = odd + 1

############# Test for Level 2 #############
aperture = 3
level = 2
denom = pow(aperture,level-1)

for row in range(0,int(denom+1)):
    print("\n=========")
    for col in range(0,int(denom-row+1)):
        print("%s;%s" % (row, col))
        cartesian_point = barycentric_to_cartesian(
                          a, b, c, 
                          (row/denom), 
                          (col/denom), 
                          ((denom - row - col)/denom))
        plt.plot(cartesian_point[0], cartesian_point[1], marker='o',
                 color="magenta")
        

plt.show()
