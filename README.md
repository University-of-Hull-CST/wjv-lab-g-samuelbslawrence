# 600086-Lab-G

In this lab we're going to extend our particle system developed in Lab-F, to add in a collision counter.

## Q1. Colliding particles

Make a copy of your `particle_threaded` project and name it `colliding_particle_threaded`

Create a new function `collide()` that checks if a particle collides with (or is very close to) another particle.

**Hint:** In terms of good object-oriented design, the `collide()` should be placed within the `Particle` class

Create a new pool of threads with a new thread main.  Within this thread main, you'll need to iterate over the list of particles calling your `collide()` function for each pair of particles.

Now add a counter to count the number of collision that occur in your simulation.  Initially, this counter can be local to the new thread main.  Print this counter before the thread terminates.
In the next exercise we'll replace this counter with an atomic.

Limit the number of collision threads to one, until you are confident that your code is executing correctly.

You should now have two sets of threads (collision and moves) that are accessing the same set of data.  One reading the other writing.

- Is locking required in your solution to prevent race conditions?
- Are there any other race conditions that can occur in your code?
- Are there any optimisations you can make to your code?

## Q2. Recording collisions using an Atomic

Make a copy of your `colliding_particle_threaded` project and name it `colliding_particle_threaded_atomic`

Replace the local counter with an atomic counter to measure the number of collisions across all threads.  This counter should be stored only once in the ParticleSystem class.

**Hint:** We covered atomic counters during the lecture in week 6

Compare the results to your original collision counters.  Is everything now working correctly?

## Q3. Ownership (optional)

Given the Rust ownership model, you have likely implemented Q1 as having a set of threads that move the particles, followed by a set of threads that test for collisions.

Can you think of an approach where both sets of threads can execute at the same time?  

This might require some lateral thinking

If you believe you have a solution, then attempt to implement it within Rust.

It's not an impossible problem, and there are likely many solutions.
