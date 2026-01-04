# Simple Physics Engine
## tldr
Simple physics engine that I made using the Barnes-Hut approximation method. The simulation puts n=500 particles on the screen with uniform mass. These particles are attracted to a center point of mass fixed in the middle of the screen. The particles are able to bounce off one another, creating a randomized visualization of particle collisions.
## Barnes-Hut Simulation
The naive approach to simulating forces on particles would be to have two nested loops in order to calculate the force between each article. This would take $O(n^2)$ operations (where $n$ is the number of particles). The Barnes-Hut approach involves approximating the center of mass for groupings of particles between one another. This reduces the number of operations to around $O(n * log n)$.
<img width="1400" height="1034" alt="image" src="https://github.com/user-attachments/assets/0096fd16-9a73-478b-87f0-551a5f10388f" />


There are several ways to tune the algorithm. The first is changing the number of particles. For this simulation, I settled for 500, but 1000 also seemed to run smoothly. Next, the value of $\theta$ is set to 0.7. $\theta$ determines the threshold for approximating a quadrant or making a new subtree. Every particle undergoes the check of (width of quadrant) / (distance to center of mass) < $\theta$. If this is true, then the quadtree is treated as single body with its own center of mass. Otherwise, keep recursing into smaller bodies. Thus, larger $\theta$ --> faster sim, smaller $\theta$ --> slower, but more accurate sim. 
<img width="1710" height="984" alt="image" src="https://github.com/user-attachments/assets/5687e12e-1592-4141-8f81-523581cc778d" />


One downside of the Barnes-Hut Simulation is that while it is fast, this is at the expense of reduced accuracy. The Fast Multipole Method (FMM) is used for situations where accuracy is crucial. Furthermore, scientists have found that Barnes-Hut can be have less than 99% accuracy in the worst case ("Skeletons from the Treecode Closet", J. Salmon and M. Warren, J. Comp. Phys. v 111, n 1, 1994).
## How The Engine Works
### Simple Walkthrough
1. Initialize SDL using Beryllium, create the render window, set bounds, etc.
2. Spawn in the particles using a random seed
3. Build the quadtree by inserting the particles (see $\theta$ explanation above)
4. Calculate the gravitational force for every body (whether that is a particle or group of particles)
5. Resolve the collisions
6. Convert the particle positions to pixel coordinates on the screen
7. Update the window buffer (make it show up on the screen)

### Calculating Gravitational Forces
Method to calculate the force between two objects:
$F = Gm_{1}m_{2} / r^{2}$, where $r^{2} = dx^{2} + dy^{2} + \epsilon^{2}$
$\epsilon^{2}$ ensures that there is not infinite force when $r=0$ during a collision.
### Tree Force Algorithm
Algorithm to calculate the force for a given quadtree:
1. If the node is a leaf, sum the forces inside the leaf pairwise.
2. Otherwise:
  3. get the nodes mass
  4. check the value of $d/r < \theta$ to decide how to approximate
    5. If true, calculate the force as one body
  6. Otherwise, recurse into the children of the subtree

### Collisions Data
<img width="600" height="371" alt="image" src="https://github.com/user-attachments/assets/d2c08087-8243-447a-94f2-76031114c5fa" />

## Future Improvements
1. Make it 3D using octotrees instead of quadtrees
2. Create random mass values for each particle
3. Mesh in methods from the Fast Multipole Method (slower but more accurate sim approach)
4. Add a UI to edit the sim in place without having to rebuild and run everytime a change is made
5. Write some more unit tests
