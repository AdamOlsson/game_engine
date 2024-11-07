# Impulse-Based Collision Resolution in 2D: Summary of Key Equations

This document provides an educational overview of the fundamental equations used for calculating impulse-based collision resolution in two-dimensional (2D) physics, specifically for the case of two colliding bodies. The purpose is to offer clear, consolidated information that can help anyone working with these types of collisions, especially as reliable and accessible resources on this topic are often scattered. This document will list the necessary equations and methods to verify them, although it won’t include detailed derivations. Instead, it will focus on the application and verification of each equation.

## Conservation of Momentum

The principle of conservation of momentum is central to understanding collision dynamics. This law states that the total momentum of a system remains constant before and after a collision, meaning the sum of the momenta of two objects before the collision is equal to the sum after. The linear momentum $ p $ of an object is given by $ p = mv $, where $ m $ is mass and $ v $ is velocity. In the case of two colliding bodies, A and B, conservation of momentum can be expressed as follows:

$$
p_{\text{pre}}^A + p_{\text{pre}}^B = p_{\text{post}}^A + p_{\text{post}}^B
$$

Since momentum is a vector quantity, it must be conserved independently in each directional component. This can be represented as:

$$
p_{\text{pre-x}}^A + p_{\text{pre-x}}^B = p_{\text{post-x}}^A + p_{\text{post-x}}^B
$$
$$
p_{\text{pre-y}}^A + p_{\text{pre-y}}^B = p_{\text{post-y}}^A + p_{\text{post-y}}^B
$$

### Angular Momentum

Angular momentum $ L $ is also conserved in collisions and includes both translational and rotational components. For a body with a mass $ m $ moving with a velocity $ v $ at a distance $ r_{CP} $ (from a reference point, often the collision point), the translational component of angular momentum is given by:

$$
L_{\text{translational}} = r_{CP} \times (mv)
$$

The rotational component, which depends on the body's moment of inertia $ I $ and angular velocity $ \omega $, is:

$$
L_{\text{rotational}} = I \omega
$$

Thus, the total angular momentum before and after a collision should be:

$$
L_{\text{pre}}^A + L_{\text{pre}}^B = L_{\text{post}}^A + L_{\text{post}}^B
$$

In practice, the vector $ r_{CP} $ can be chosen relative to the collision point, but simplifying assumptions (e.g., using the origin) may be made for easier calculations. The above equations can be used to verify the correctness of the below equations. Consider using these in any unit tests you implement.

## Calculating Impulse Magnitude

During a collision, the total impulse $ J $ exchanged between two bodies at the point of impact determines the resulting changes in their velocities. Impulse considers both linear and angular contributions from each object, and it can be computed as follows:

$$
J = \frac{-(1 + e) \, v_{\text{pre}}^{\text{relative}} \cdot n}
{n \cdot n \left( \frac{1}{m^A} + \frac{1}{m^B} \right) + \frac{(r^{AP} \times n)^2}{I^A} + \frac{(r^{BP} \times n)^2}{I^B}}
$$

Here, the terms are defined as:

- $ e $: coefficient of restitution, measuring the "elasticity" of the collision
- $ P $: contact point between the two objects
- $ v_{\text{pre}}^{\text{relative}} $: relative velocity between objects $ A $ and $ B $ at the contact point $ P $ before collision
- $ n $: collision normal vector directed toward object $ A $
- $ I $: moment of inertia of the object
- $ r^{AP} $: vector from the center of mass of object $ A $ to the collision point $ P $

## Calculating Relative Velocity at Contact Point

The relative velocity at the point of contact, $ v_{\text{pre}}^{\text{relative}} $, considers both linear and angular velocities of each object. For an object, the velocity at the collision point can be described as:

$$
v_{\text{pre}}^{AP} = v_{\text{linear}}^A + \omega_{\text{pre}} \, r_{\perp}^{AP}
$$
$$
v_{\text{pre}}^{BP} = v_{\text{linear}}^B + \omega_{\text{pre}} \, r_{\perp}^{BP}
$$
$$
v_{\text{pre}}^{\text{relative}} = v_{\text{pre}}^{AP} - v_{\text{pre}}^{BP}
$$

where $ r_{\perp}^{AP} $ is the vector perpendicular to $ r^{AP} $, and $ \omega $ is the angular velocity.

## Updating Linear and Angular Velocities

After determining the impulse $ J $, it can be used to update the post-collision velocities for both linear and angular components:

$$
v_{\text{post}} = v_{\text{pre}} + \frac{J}{m} n
$$
$$
\omega_{\text{post}} = \omega_{\text{pre}} + \frac{r_{\perp}^{AP} \cdot J \, n}{I}
$$

These equations provide the new linear and angular velocities of the objects, allowing for the accurate simulation of 2D collision outcomes in physics engines and simulations.


# Sources
- https://chrishecker.com/Rigid_Body_Dynamics
- https://chrishecker.com/images/c/c2/Gdmphys2.pdf
- https://chrishecker.com/images/e/e7/Gdmphys3.pdf
- https://phys.libretexts.org/Courses/Prince_Georges_Community_College/PHY_1030%3A_General_Physics_I/07%3A_Linear_Momentum_and_Collisions/7.3%3A_Collisions
- https://www.sparknotes.com/physics/linearmomentum/collisions/section2/
- https://www2.tntech.edu/leap/murdock/books/v1chap7.pdf
- https://research.ncl.ac.uk/game/mastersdegree/gametechnologies/physicstutorials/5collisionresponse/Physics%20-%20Collision%20Response.pdf
- https://mechanicsmap.psu.edu/websites/15_impulse_momentum_rigid_body/15-2_impulse_momentum_theorem_rigid_body/impulse_momentum_theorem_rigid_body.html
- https://mechanicsmap.psu.edu/websites/15_impulse_momentum_rigid_body/15-4_rigid_body_free_collisions/free_rigid_body_collisions.html