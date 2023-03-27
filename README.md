# Infinite Diffusion Limited Aggregation Simulator

This software simulates DLA while continuously increasing the simulation space to not limit the aggregate's growth. 

### By tweaking the parameters different shapes and colors of an aggregate can be achieved
![Screenshot of a circular aggregate](/images/img1.png)
![Screenshot of an irregular aggregate](/images/img2.png)
![Screenshot of branches](/images/img8.png)

### To remain performant at larger scales, the simulator uses bins spatial partitioning
![Screenshot of a bins data structure](/images/img4.png)

### Different display modes are supported
![Screenshot of particles](/images/img6.png)
![Screenshot of branches](/images/img7.png)

### UI gives the user full control over the simulation
![Screenshot of UI](/images/img5.png)

**Seed color** - The initial color of the center particle

**Color variation** - How much neighbouring particles' colors are allowed to differ

**Branch thickness** - When rendering branches, how thick they should be

**Iterations per frame** - The speed of the simulation

**Particles count** - The amount of particles that are moving around the aggregate

**Zoom smoothness** - How fluid the camera movements should be

**World-aggregate ratio** - How much bigger the simulation space should be in coma=parison to the aggregate

**Zoom-aggregate ratio** - How much bigger the viewport should be than the aggregate

**Particle radius** - The size of a particle. Affects both dynamic particles and particles in the aggregate.

**Branch animation time** - How fast the animation of a growing branch should be

**Bin margin min** - When the aggregate gets this close to the spatial acceleration structure's edge, it should resize

**Bin margin max** - When the bins structure is resized, the new margin from the aggregate should be this big 