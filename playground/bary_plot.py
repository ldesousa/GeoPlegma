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


def denominator_even(aperture, level):

    print("level:%s aperture:%s" % (level, aperture))
    if level < 3:
        return 3
    return int(pow(aperture,2) * pow(2,(level-aperture-1)/2))


def denominator_odd(aperture, level):

    return int(pow(aperture,2) * pow(2,(level-aperture)/2))


def plot_even(a, b, c, aperture, level, colour):

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
            #plot_even_cell(a,b,c,aperture,level,colour,row,col)
            

def plot_even_cell(a, b, c, aperture, level, colour, i, j):

    vec_i = [2,1,-1,-2,-1,1]
    vec_j = [-1,1,2,1,-1,-2]

    denom = denominator_even(aperture,level)
    #denom = 3
    next_denom = denominator_odd(aperture,level+1)
    print("denom: %s" % (str(denom)))
    print("next_denom: %s" % (str(next_denom)))
    
    next_i = next_denom * i / denom
    next_j = next_denom * j / denom
    print("next_i:%s next_j:%s" % (next_i, next_j))

    vec_i = [x + next_i for x in vec_i] 
    vec_j = [x + next_j for x in vec_j] 
    print("Vec i: %s" % (str(vec_i)))
    print("Vec j: %s" % (str(vec_j)))

    cartesians_x = []
    cartesians_y = []
    for x in range (len(vec_i)):
        cartesian_point = barycentric_to_cartesian(
                              a, b, c, 
                              (vec_i[x]/next_denom), 
                              (vec_j[x]/next_denom), 
                              ((next_denom - vec_i[x] - vec_j[x])/next_denom))
        cartesians_x.append(cartesian_point[0])
        cartesians_y.append(cartesian_point[1])
        print("Bary coords: %s; %s; %s (over %s)" % (vec_i[x] ,vec_j[x] ,str(next_denom - vec_i[x] - vec_j[x]), next_denom))

    plt.plot(cartesians_x, cartesians_y, color=colour, linestyle='dashed')


def plot_odd_cell(a, b, c, aperture, level, colour, i, j):

    vec_i = [1,0,-1,-1,0,1]
    vec_j = [0,1,1,0,-1,-1]

    denom = denominator_odd(aperture,level)
#    print("denom: %s" % (str(denom)))
    #print("next_denom: %s" % (str(next_denom)))
    
    vec_i = [x + i for x in vec_i] 
    vec_j = [x + j for x in vec_j] 

 #   print("Vec i: %s" % (str(vec_i)))
  #  print("Vec j: %s" % (str(vec_j)))

    cartesians_x = []
    cartesians_y = []
    for x in range (len(vec_i)):
        cartesian_point = barycentric_to_cartesian(
                              a, b, c, 
                              (vec_i[x]/denom), 
                              (vec_j[x]/denom), 
                              ((denom - vec_i[x] - vec_j[x])/denom))
   #     print("Bary coords: %s; %s; %s" % (str(vec_i[x]/denom),str(vec_j[x]/denom),str((denom - vec_i[x] - vec_j[x])/denom)))
        cartesians_x.append(cartesian_point[0])
        cartesians_y.append(cartesian_point[1])
    #print("cartesians_x: %s" % (str(cartesians_x)))
    #print("cartesians_y: %s" % (str(cartesians_y)))

    cartesians_x.append(cartesians_x[0])
    cartesians_y.append(cartesians_y[0])

    plt.plot(cartesians_x, cartesians_y, color=colour, linestyle='dashed')



def plot_odd(a, b, c, aperture, level, colour):
    
    denom = denominator_odd(aperture, level)
    
    for i in range(denom+1):
        start = i % level
        for j in range(start,denom+1-i,3):
                cartesian_point = barycentric_to_cartesian(
                                  a, b, c, 
                                  (i/denom), 
                                  (j/denom), 
                                  ((denom - i - j)/denom))
                plt.plot(cartesian_point[0], cartesian_point[1], marker='o',
                         markersize=9, color=colour)


def plot_lines(a, b, c, aperture, level):
    nlines = int(pow(aperture,2) * pow(2,(level-aperture)/2) )
    
    for i in range(1,nlines):
        bcA = barycentric_to_cartesian(a,b,c,i/nlines,0,     1-i/nlines)
        bcB = barycentric_to_cartesian(a,b,c,i/nlines,1-i/nlines,0)
        plt.plot([bcA[0],bcB[0]],[bcA[1],bcB[1]], markersize=1, color="grey")
      
        bcA = barycentric_to_cartesian(a,b,c,0,     1-i/nlines,i/nlines)
        bcB = barycentric_to_cartesian(a,b,c,1-i/nlines,0,     i/nlines)
        plt.plot([bcA[0],bcB[0]],[bcA[1],bcB[1]], markersize=1, color="grey")
      
        bcA = barycentric_to_cartesian(a,b,c,0,     i/nlines,1-i/nlines)
        bcB = barycentric_to_cartesian(a,b,c,1-i/nlines,i/nlines,0)
        plt.plot([bcA[0],bcB[0]],[bcA[1],bcB[1]], markersize=1, color="grey")

            
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
#plt.grid(True)
plt.axis('equal')  # To keep aspect ratio equal

# Barycentric coordinates
u, v, w = (1.0/3), (1.0/3), (1.0/3)

cartesian_point = barycentric_to_cartesian(a, b, c, u, v, w)
plt.plot(cartesian_point[0], cartesian_point[1], marker='o')  # Optional: add markers at point


aperture = 3

############# Level lines ###################

level = 3
#plot_lines(a, b, c, aperture, level)


############# Plot Cells ####################

#plot_even_cell(a, b, c, aperture, 2, "blue", 0, 0)
#plot_even_cell(a, b, c, aperture, 2, "blue", 0, 1)
#plot_even_cell(a, b, c, aperture, 2, "blue", 0, 2)
#plot_even_cell(a, b, c, aperture, 2, "blue", 0, 3)
#plot_even_cell(a, b, c, aperture, 2, "blue", 1, 0)
plot_even_cell(a, b, c, aperture, 2, "blue", 1, 1)
#plot_even_cell(a, b, c, aperture, 2, "blue", 1, 2)
#plot_even_cell(a, b, c, aperture, 2, "blue", 2, 0)
#plot_even_cell(a, b, c, aperture, 2, "blue", 2, 1)
#plot_even_cell(a, b, c, aperture, 2, "blue", 3, 0)

plot_even_cell(a, b, c, aperture, 4, "magenta", 1, 0)



#plot_odd_cell(a,b,c, aperture, 3, "orange", 3, 3)
#plot_odd_cell(a,b,c, aperture, 3, "orange", 4, 1)
#plot_odd_cell(a,b,c, aperture, 3, "orange", 1, 4)
#plot_odd_cell(a,b,c, aperture, 3, "orange", 5, 2)
#plot_odd_cell(a,b,c, aperture, 3, "orange", 2, 5)


plot_odd_cell(a,b,c, aperture, 3, "orange", 0, 0)
plot_odd_cell(a,b,c, aperture, 3, "orange", 0, 3)
plot_odd_cell(a,b,c, aperture, 3, "orange", 0, 6)
plot_odd_cell(a,b,c, aperture, 3, "orange", 0, 9)

plot_odd_cell(a,b,c, aperture, 3, "orange", 1, 1)
plot_odd_cell(a,b,c, aperture, 3, "orange", 1, 4)
plot_odd_cell(a,b,c, aperture, 3, "orange", 1, 7)


plot_odd_cell(a,b,c, aperture, 3, "orange", 2, 2)
plot_odd_cell(a,b,c, aperture, 3, "orange", 2, 5)
#plot_odd_cell(a,b,c, aperture, 3, "orange", 2, 8)

plot_odd_cell(a,b,c, aperture, 3, "orange", 3, 0)
plot_odd_cell(a,b,c, aperture, 3, "orange", 3, 3)
plot_odd_cell(a,b,c, aperture, 3, "orange", 3, 6)

plot_odd_cell(a,b,c, aperture, 3, "orange", 4, 1)
plot_odd_cell(a,b,c, aperture, 3, "orange", 4, 4)

plot_odd_cell(a,b,c, aperture, 3, "orange", 5, 2)

plot_odd_cell(a,b,c, aperture, 3, "orange", 6, 0)
plot_odd_cell(a,b,c, aperture, 3, "orange", 6, 3)

plot_odd_cell(a,b,c, aperture, 3, "orange", 7, 1)

plot_odd_cell(a,b,c, aperture, 3, "orange", 9, 0)
        
############# Test for Odd Level 3 #############
level = 3
plot_odd(a, b, c, aperture, level, "orange")


############# Test for Even level #############
plot_even(a, b, c, aperture, 4, "magenta")
#plot_even(a, b, c, aperture, 2, "blue")

plt.show()
