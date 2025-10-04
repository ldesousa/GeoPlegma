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


def plot_even(aperture, level, colour):

    #denom = pow(aperture,level-1)
    denom = aperture * (level-1)
    
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
                     color=colour)

            
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
level = 5
# denom = pow(aperture,level-1)
denom = pow(aperture,2) * (level-2)
odd = 0

num_hops = denom // level + 1

for row in range(0,int(denom+1)):
    col_start = row % level
    print("\n========= row:%s, start:%s, hops:%s" % (str(row),str(col_start),int(num_hops)))
    for col in range(0,int(num_hops)):
        j = (col_start + (col*level))
        print("%s;%s" % (row, j))
        cartesian_point = barycentric_to_cartesian(
                          a, b, c, 
                          (row/denom), 
                          (j/denom), 
                          ((denom - row - j)/denom))
        plt.plot(cartesian_point[0], cartesian_point[1], marker='o',
                 markersize=12, color="orange")

    if ((row+1) % level) == 0:
        num_hops = num_hops + 1
    else:
        num_hops = num_hops - 1


############# Test for Even level #############
plot_even(3, 4, "magenta")
plot_even(3, 2, "blue")
        

plt.show()
