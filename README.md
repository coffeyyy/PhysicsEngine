# Simple Physics Engine
## tldr
Simple physics engine that I made using the Barnes-Hut approximation method. The simulation puts n=500 particles on the screen with uniform mass. These particles are attracted to a center point of mass fixed in the middle of the screen. The particles are able to bounce off one another, creating a randomized visualization of particle collisions.
## Barnes-Hut Simulation
The naive approach to simulating forces on particles would be to have two nested loops in order to calculate the force between each article. This would take $O(n^2)$ operations (where $n$ is the number of particles). The Barnes-Hut approach involves approximating the center of mass for groupings of particles between one another. This reduces the number of operations to around $O(n * log n)$.

There are several ways to tune the algorithm. The first is changing the number of particles. For this simulation, I settled for 500, but 1000 also seemed to run smoothly. Next, the value of $\theta$ is set to 0.7. $\theta$ determines the threshold for approximating a quadrant or making a new subtree. Every particle undergoes the check of (width of quadrant) / (distance to center of mass) < $\theta$. If this is true, then the quadtree is treated as single body with its own center of mass. Otherwise, keep recursing into smaller bodies. Thus, larger $\theta$ --> faster sim, smaller $\theta$ --> slower, but more accurate sim. 
## How The Engine Works
## Challenges
### Problem #1: Managing internal state for each quadtree
### Problem #2: Getting the particles actually on the screen
### Problem #3: Collisions were way too violent - particles would shoot into the oblivion
## Future Improvements
1. Make it 3D using octotrees instead of quadtrees
2. Mesh in methods from the Fast Multipole Method (slower but more accurate sim method)
3. Add a UI to edit the sim in place without having to rebuild and run everytime a change is made
